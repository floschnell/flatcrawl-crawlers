extern crate chrono;
extern crate config;
extern crate dns_lookup;
extern crate futures;
extern crate kuchiki;
extern crate lapin_futures as lapin;
extern crate reqwest;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate tokio_core;

mod crawlers;
mod models;
mod configuration;

use std::boxed::Box;
use std::thread;
use std::sync::{Arc, Barrier};
use lapin::client::ConnectionOptions;
use lapin::channel::{BasicProperties, BasicPublishOptions};
use tokio_core::reactor::Core;
use tokio_core::net::TcpStream;
use futures::Future;
use std::sync::Mutex;
use dns_lookup::lookup_host;
use crawlers::Crawler;
use models::Flat;

fn main() {
    let conf = configuration::read();
    let amqp_host = conf.amqp_config.host.to_owned();
    let thread_count = conf.thread_count as usize;

    let barrier = Arc::new(Barrier::new(thread_count + 1));
    let amqp_host_ip = lookup_host(amqp_host.as_str()).unwrap()[0];
    let mut last_flats = Vec::<Flat>::new();
    loop {
        let crawlers = crawlers::get_crawlers();
        let guarded_crawlers = Arc::new(Mutex::new(crawlers));
        let guarded_flats = Arc::new(Mutex::new(Vec::<Flat>::new()));

        // process all crawlers
        for i in 0..thread_count {
            let inner_guarded_crawlers = guarded_crawlers.clone();
            let inner_guarded_flats = guarded_flats.clone();
            let inner_barrier = barrier.clone();
            thread::spawn(move || {
                run_thread(inner_guarded_crawlers, inner_guarded_flats, i);
                inner_barrier.wait();
            });
        }

        // wait for all threads to finish
        barrier.wait();

        // filter results for duplicates
        let flats = Arc::try_unwrap(guarded_flats)
            .unwrap()
            .into_inner()
            .unwrap();
        println!("Successfully parsed {} flats.", flats.len());
        let mut iterable_last_flats = last_flats.into_iter();
        let filtered_flats: Vec<_> = flats
            .to_vec()
            .into_iter()
            .filter(|flat| {
                iterable_last_flats.all(|old| old.id().unwrap() != flat.id().unwrap())
            })
            .collect();

        // only send new flats
        println!("Will be sending {} flats ...", filtered_flats.len());
        send_results(&conf.amqp_config, amqp_host_ip, &filtered_flats);
        println!("Done.");
        last_flats = flats.to_vec();

        // pause for 5 minutes
        std::thread::sleep(std::time::Duration::from_secs(300));
    }
}

fn get_crawler(guarded_crawlers: &Arc<Mutex<Vec<Box<Crawler>>>>) -> Option<Box<Crawler>> {
    let mut crawlers = guarded_crawlers.lock().unwrap();
    crawlers.pop()
}

fn add_flats(guarded_flats: &Arc<Mutex<Vec<Flat>>>, in_flats: &mut Vec<Flat>) {
    let mut flats = guarded_flats.lock().unwrap();
    flats.append(in_flats);
}

fn run_thread(
    guarded_crawlers: Arc<Mutex<Vec<Box<Crawler>>>>,
    guarded_flats: Arc<Mutex<Vec<Flat>>>,
    thread_number: usize,
) {
    loop {
        let crawler_opt = get_crawler(&guarded_crawlers);
        if crawler_opt.is_some() {
            let crawler = crawler_opt.unwrap();
            println!(
                "processing '{}' on thread {} ...",
                crawler.name(),
                thread_number
            );
            let flats_result = crawler.crawl();
            if flats_result.is_ok() {
                add_flats(&guarded_flats, &mut flats_result.unwrap());
            } else {
                println!("error: {:?}", flats_result.err().unwrap().message).to_owned();
            }
        } else {
            break;
        }
    }
}

fn send_results(
    config: &configuration::AmqpConfig,
    ip_addr: std::net::IpAddr,
    results: &Vec<Flat>,
) {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let socket = std::net::SocketAddr::new(ip_addr, 5672);

    core.run(
        TcpStream::connect(&socket, &handle)
            .and_then(|stream| {
                let mut options = ConnectionOptions::default();
                options.username = config.username.to_owned();
                options.password = config.password.to_owned();
                lapin::client::Client::connect(stream, &options)
            })
            .and_then(|(client, _)| client.create_channel())
            .and_then(|channel| {
                for flat in results {
                    channel.basic_publish(
                        "",
                        config.queue.to_owned().as_str(),
                        serde_json::to_string(&flat).unwrap().as_bytes(),
                        &BasicPublishOptions::default(),
                        BasicProperties::default(),
                    );
                }
                Ok(())
            }),
    ).unwrap();
}
