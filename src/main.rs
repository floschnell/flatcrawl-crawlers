mod configuration;
mod crawlers;
mod geocode;
mod models;

use crate::lapin::options::{BasicPublishOptions, ExchangeDeclareOptions};
use crate::lapin::types::FieldTable;
use crate::lapin::{BasicProperties, Client, ConnectionProperties, ExchangeKind};
use crate::models::Flat;
use configuration::ApplicationConfig;
use crawlers::Config;
use futures::future::Future;
use lapin_futures as lapin;
use std::sync::Mutex;
use std::sync::{Arc, Barrier};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;

fn main() {
  let app_config = configuration::read();
  let mut init_run = if app_config.test { false } else { true };
  let amqp_host = app_config.amqp_config.host.to_owned();
  let thread_count = app_config.thread_count as usize;

  if app_config.test {
    println!("----- Running in TEST mode! -----");
    let flats = vec![Flat {
      city: models::City::Munich,
      source: "immoscout".to_owned(),
      location: Some(models::Location {
        latitude: 9.0,
        longitude: 10.0,
        uncertainty: 0.0,
      }),
      data: Some(models::FlatData {
        address: "Some address".to_owned(),
        externalid: "4".to_owned(),
        rent: 100.0,
        rooms: 2.0,
        squaremeters: 60.0,
        title: "Test Flat".to_owned(),
      }),
      date: 0,
    }];
    println!("flat: {}", serde_json::to_string(&flats[0]).unwrap());
    send_results(&app_config, amqp_host.as_str(), flats);
  }

  let barrier = Arc::new(Barrier::new(thread_count + 1));
  let mut last_flats = Vec::<Flat>::new();
  loop {
    let crawl_start = Instant::now();
    let guarded_configs = Arc::new(Mutex::new(crawlers::get_crawler_configs()));

    // process all crawlers
    let mut thread_handles: Vec<JoinHandle<Vec<Flat>>> = vec![];
    for i in 0..thread_count {
      let inner_guarded_configs = guarded_configs.clone();
      let inner_barrier = barrier.clone();
      let cap_conf = app_config.clone();
      let handle = thread::spawn(move || {
        let flats = run_thread(inner_guarded_configs, i, &cap_conf);
        inner_barrier.wait();
        flats
      });
      &mut thread_handles.push(handle);
    }

    // wait for all threads to finish
    barrier.wait();

    // collect results
    let flats = thread_handles
      .into_iter()
      .map(|h| h.join().unwrap_or_default())
      .flatten()
      .collect::<Vec<_>>();

    // filter results for duplicates
    let mut filtered_flats: Vec<_> = Vec::new();
    println!("successfully parsed {} flats.", flats.len());
    for current_flat in flats.to_vec() {
      let has_been_sent = last_flats
        .to_vec()
        .into_iter()
        .any(|previous_flat| previous_flat == current_flat);
      if !has_been_sent {
        filtered_flats.push(current_flat);
      }
    }

    let run_duration = crawl_start.elapsed();
    println!(
      "analyzed {} pages and found {} flats in {}.{} seconds.",
      crawlers::get_crawler_configs().len(),
      flats.len(),
      run_duration.as_secs(),
      run_duration.subsec_millis()
    );

    // in the first run, we will collect
    if init_run {
      init_run = false;
      println!("during initial run, we do not send flats ...");
    } else {
      // geocode all new flats
      let geocoded_flats = geocode_flats(&filtered_flats, &app_config);

      // only send new flats
      if app_config.test {
        for flat in geocoded_flats {
          println!("flat that would be send: {:?}", flat);
          println!("run finished.");
        }
      } else {
        println!("will be sending {} flats ...", geocoded_flats.len());
        send_results(&app_config, amqp_host.as_str(), geocoded_flats);
        println!("done.");
      }
    }

    // remember the flats so we can compare against them
    // during the next run ...
    last_flats = flats.to_vec();

    // pause for 5 minutes
    std::thread::sleep(std::time::Duration::from_secs(300));
  }
}

fn run_thread(
  guarded_configs: Arc<Mutex<Vec<Config>>>,
  thread_number: usize,
  conf: &ApplicationConfig,
) -> Vec<Flat> {
  let mut flats: Vec<Flat> = vec![];
  loop {
    let config_opt = guarded_configs.lock().unwrap().pop();
    match config_opt {
      Some(config) => {
        flats.append(&mut process_config(&conf, &config, thread_number));
      }
      None => break,
    }
  }
  flats
}

fn geocode_flats(results: &Vec<Flat>, config: &ApplicationConfig) -> Vec<Flat> {
  let mut enriched_flats = Vec::new();
  for flat in results {
    let geocode_result_opt = match &flat.data {
      Some(data) => match geocode::geocode(&config.nominatim_url, &data.address) {
        Ok(coords) => Some(coords),
        Err(_) => None,
      },
      None => None,
    };
    let enriched_flat = match geocode_result_opt {
      Some(geocode_result) => flat.locate(&geocode_result.coord, geocode_result.uncertainty),
      None => flat.clone(),
    };
    enriched_flats.push(enriched_flat);
  }
  enriched_flats
}

fn process_config(
  app_config: &ApplicationConfig,
  crawl_config: &Config,
  thread_number: usize,
) -> Vec<Flat> {
  let crawler = crawlers::get_crawler(&crawl_config.crawler);
  match crawler {
    Ok(crawler) => {
      println!(
        "processing '{}' on thread {} ...",
        crawler.name(),
        thread_number
      );
      let flats_result = crawlers::execute(crawl_config, &crawler);
      if flats_result.is_ok() {
        let flats = flats_result.unwrap();
        if app_config.test {
          for ref flat in &flats {
            println!("parsed flat: {:?}", flat);
          }
        }
        flats
      } else {
        eprintln!("error: {:?}", flats_result.err().unwrap().message);
        vec![]
      }
    }
    Err(e) => {
      eprintln!("config could not be processed: {:?}", e.message);
      vec![]
    }
  }
}

fn send_results(app_config: &ApplicationConfig, host: &str, results: Vec<Flat>) {
  let exchange = if app_config.test {
    "test_flats_exchange"
  } else {
    "flats_exchange"
  };
  let mut address = String::from("amqp://");
  address.push_str(app_config.amqp_config.username.as_str());
  address.push_str(":");
  address.push_str(app_config.amqp_config.password.as_str());
  address.push_str("@");
  address.push_str(host);
  address.push_str(":5672/%2f");

  let connection = Client::connect(address.as_str(), ConnectionProperties::default())
    .wait()
    .expect("connection error");

  let channel = connection
    .create_channel()
    .wait()
    .expect("channel could not be created");

  println!("connection successfully established");

  channel
    .exchange_declare(
      exchange,
      ExchangeKind::Fanout,
      ExchangeDeclareOptions::default(),
      FieldTable::default(),
    )
    .wait()
    .expect("could not create exchange");

  println!("exchange successfully created");

  print!("sending flats ");
  for flat in results {
    channel
      .basic_publish(
        exchange,
        &format!("flats_{:?}", flat.city),
        serde_json::to_string(&flat).unwrap().as_bytes().to_vec(),
        BasicPublishOptions::default(),
        BasicProperties::default(),
      )
      .wait()
      .expect("could not send flat!");
    print!(".");
  }
  println!();

  println!("sending flats complete.");
}
