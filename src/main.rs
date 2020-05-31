mod configuration;
mod crawlers;
mod geocode;
mod models;

use crate::models::{Property, PropertyData, PropertyType, ContractType};
use configuration::ApplicationConfig;
use crawlers::Config;
use std::sync::Mutex;
use std::sync::{Arc, Barrier};
use std::thread;
use std::thread::JoinHandle;
use std::time::Instant;
use firestore_db_and_auth::{Credentials, ServiceSession, documents, errors};

fn main() {
  let app_config = configuration::read();
  let thread_count = app_config.thread_count as usize;

  println!("connecting to firebase ...");
  let cred = Credentials::from_file("flatcrawl-firebase.json").expect("Read firebase credentials file");
  let session = ServiceSession::new(cred).expect("Create firebase service account session");
  println!("connection successfully established.");

  if app_config.test {
    println!("----- Running in TEST mode! -----");
    let flats = vec![Property {
      city: models::City::Munich,
      source: "immoscout".to_owned(),
      location: Some(models::Location {
        latitude: 9.0,
        longitude: 10.0,
        uncertainty: 0.0,
      }),
      data: Some(PropertyData {
        address: "Some address".to_owned(),
        externalid: "4".to_owned(),
        price: 100.0,
        rooms: 2.0,
        squaremeters: 60.0,
        title: "Test Flat".to_owned(),
        contract_type: ContractType::Rent,
        property_type: PropertyType::Flat,
      }),
      date: 0,
    }];
    println!("flat: {}", serde_json::to_string(&flats[0]).unwrap());
    send_results(&session, flats);
  }

  let barrier = Arc::new(Barrier::new(thread_count + 1));
  let mut last_flats = Vec::<Property>::new();
  loop {
    let crawl_start = Instant::now();
    let guarded_configs = Arc::new(Mutex::new(crawlers::get_crawler_configs()));

    // process all crawlers
    let mut thread_handles: Vec<JoinHandle<Vec<Property>>> = vec![];
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

    let run_duration = crawl_start.elapsed();
    println!(
      "analyzed {} pages and found {} flats in {}.{} seconds.",
      crawlers::get_crawler_configs().len(),
      flats.len(),
      run_duration.as_secs(),
      run_duration.subsec_millis()
    );

    // filter results for duplicates
    let mut filtered_flats: Vec<_> = Vec::new();
    println!("Before deduplication: {}", flats.len());
    for current_flat in flats.to_vec() {
      let has_been_sent = last_flats
        .to_vec()
        .into_iter()
        .any(|previous_flat| previous_flat == current_flat);
      if !has_been_sent {
        filtered_flats.push(current_flat);
      }
    }

    println!("After offline deduplication: {}", filtered_flats.len());

    let mut new_flats: Vec<Property> = vec!();
    for ref flat in filtered_flats {
      let id = flat.data.as_ref().map(|x| x.externalid.to_owned());
      let document_id = id.map(|x| format!("{}-{}", flat.source, x)).unwrap();
      let document: Result<Property, errors::FirebaseError> = documents::read(&session, "flats", document_id);
      match document {
        Ok(_) => (),
        Err(_) => new_flats.push(flat.to_owned()),
      }
    }
    
    println!("After online deduplication: {}", new_flats.len());

    // geocode all new flats
    let geocoded_flats = geocode_flats(&new_flats, &app_config);

    // send flats
    if app_config.test {
      for flat in geocoded_flats {
        println!("flat that would be send: {:?}", flat);
        println!("run finished.");
      }
    } else {
      send_results(&session, geocoded_flats);
      println!("done.");
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
) -> Vec<Property> {
  let mut flats: Vec<Property> = vec![];
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

fn geocode_flats(results: &Vec<Property>, config: &ApplicationConfig) -> Vec<Property> {
  let mut enriched_flats = Vec::new();
  print!("geocoding flats ...");
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
    print!(".");
  }
  println!();
  enriched_flats
}

fn process_config(
  app_config: &ApplicationConfig,
  crawl_config: &Config,
  thread_number: usize,
) -> Vec<Property> {
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

fn send_results(session: &ServiceSession, results: Vec<Property>) {
  print!("sending flats ...");
  for flat in results {
    let id = flat.data.as_ref().map(|x| x.externalid.to_owned());
    let document_id = id.map(|x| format!("{}-{}", flat.source, x));
    let result = documents::write(session, "flats", document_id, &flat, documents::WriteOptions::default());
    match result.err() {
      Some(error) => println!("ERROR: {:?}!", error),
      None => print!(".")
    }
  }
  println!();
}
