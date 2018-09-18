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

mod configuration;
mod crawlers;
mod geocode;
mod models;

use crawlers::Crawler;
use dns_lookup::lookup_host;
use futures::Future;
use lapin::channel::{BasicProperties, BasicPublishOptions};
use lapin::client::ConnectionOptions;
use models::Flat;
use std::boxed::Box;
use std::sync::Mutex;
use std::sync::{Arc, Barrier};
use std::thread;
use tokio_core::net::TcpStream;
use tokio_core::reactor::Core;

fn main() {
  let conf = configuration::read();
  let mut init_run = if conf.test { false } else { true };
  let amqp_host = conf.amqp_config.host.to_owned();
  let thread_count = conf.thread_count as usize;

  if conf.test {
    println!("----- Running in TEST mode! -----");
  }

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
      let cap_conf = conf.clone();
      thread::spawn(move || {
        run_thread(inner_guarded_crawlers, inner_guarded_flats, i, &cap_conf);
        inner_barrier.wait();
      });
    }

    // wait for all threads to finish
    barrier.wait();

    // filter results for duplicates
    let mut filtered_flats: Vec<_> = Vec::new();
    let flats = Arc::try_unwrap(guarded_flats)
      .unwrap()
      .into_inner()
      .unwrap();
    println!("Successfully parsed {} flats.", flats.len());
    for flat in flats.to_vec() {
      let has_been_sent = last_flats
        .to_vec()
        .into_iter()
        .any(|previous| previous.id().unwrap() == flat.id().unwrap());
      if !has_been_sent {
        filtered_flats.push(flat);
      }
    }

    // in the first run, we will collect
    if init_run {
      init_run = false;
      println!("During initial run, we do not send flats ...");
    } else {
      // geocode all new flats
      let geocoded_flats = geocode_flats(&filtered_flats, &conf);

      // only send new flats
      if conf.test {
        for flat in geocoded_flats {
          println!("Flat that would be send: {:?}", flat);
          println!("Run finished.");
        }
      } else {
        println!("Will be sending {} flats ...", geocoded_flats.len());
        send_results(&conf.amqp_config, amqp_host_ip, &geocoded_flats);
        println!("Done.");
      }
    }

    // remember the flats so we can compare against them
    // during the next run ...
    last_flats = flats.to_vec();

    // pause for 5 minutes
    std::thread::sleep(std::time::Duration::from_secs(300));
  }
}

fn geocode_flats(results: &Vec<Flat>, config: &configuration::CrawlerConfig) -> Vec<Flat> {
  let mut enriched_flats = Vec::new();
  for flat in results {
    let coords_opt = match &flat.data {
      Some(data) => match geocode::geocode(&config.nominatim_url, &data.address) {
        Ok(coords) => Some(coords),
        Err(_) => None,
      },
      None => None,
    };
    let enriched_flat = match coords_opt {
      Some(coords) => flat.locate(&coords),
      None => flat.clone(),
    };
    enriched_flats.push(enriched_flat);
  }
  enriched_flats
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
  conf: &configuration::CrawlerConfig,
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
        let mut flats = flats_result.unwrap();
        if conf.test {
          for ref flat in &flats {
            println!("Parsed flat: {:?}", flat);
          }
        }
        add_flats(&guarded_flats, &mut flats);
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

  core
    .run(
      TcpStream::connect(&socket, &handle)
        .and_then(|stream| {
          let mut options = ConnectionOptions::default();
          options.username = config.username.to_owned();
          options.password = config.password.to_owned();
          lapin::client::Client::connect(stream, options)
        })
        .and_then(|(client, _)| client.create_channel())
        .and_then(|channel| {
          for flat in results {
            channel
              .basic_publish(
                "",
                config.queue.to_owned().as_str(),
                serde_json::to_string(&flat).unwrap().as_bytes().to_vec(),
                BasicPublishOptions::default(),
                BasicProperties::default(),
              )
              .map(|confirmation| println!("publish got confirmation: {:?}", confirmation))
              .and_then(|_| channel.close(200, "Bye"))
              .wait()
              .expect("Ok");
          }
          Ok(())
        }),
    )
    .unwrap();
}
