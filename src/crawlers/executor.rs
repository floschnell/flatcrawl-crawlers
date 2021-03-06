extern crate encoding_rs;
extern crate kuchiki;
extern crate regex;
extern crate reqwest;
extern crate std;

use crate::crawlers::{Config, Crawler, Error as CrawlingError};
use crate::models::{Encoding, Flat};
use kuchiki::iter::*;
use kuchiki::traits::*;
use reqwest::Response;
use std::error::Error as StdErr;

#[derive(Debug)]
pub struct Error {
  pub message: String,
}

impl From<CrawlingError> for Error {
  fn from(err: CrawlingError) -> Error {
    return Error {
      message: format!("Crawler Error: {}", err.message),
    };
  }
}

impl From<std::io::Error> for Error {
  fn from(_err: std::io::Error) -> Error {
    return Error {
      message: "IO Error".to_owned(),
    };
  }
}

impl From<reqwest::Error> for Error {
  fn from(_err: reqwest::Error) -> Error {
    let mut message = String::from("Request Error: ");
    message.push_str(_err.description());
    return Error { message };
  }
}

pub fn execute(config: &Config, crawler: &Box<dyn Crawler>) -> Result<Vec<Flat>, Error> {
  let results = get_results(config, crawler)?;
  let mut successful: Vec<Flat> = Vec::new();
  let flat_results: Vec<Result<Flat, Error>> = results
    .map(|result| {
      let flat = Flat::new(crawler.name().to_owned(), config.city.clone());
      let data = crawler.transform_result(result)?;
      Ok(flat.fill(&data))
    })
    .collect();
  for flat_result in flat_results {
    match flat_result {
      Ok(flat) => successful.push(flat),
      Err(e) => println!(
        "Could not process flat within crawler '{}', because: {}",
        crawler.name(),
        e.message
      ),
    }
  }
  Ok(successful)
}

fn decode_response(response: &mut Response, encoding: &Encoding) -> Result<String, Error> {
  let mut buf: Vec<u8> = vec![];
  response.copy_to(&mut buf)?;
  let (encoded_string, _, _) = match encoding {
    Encoding::Latin1 => encoding_rs::ISO_8859_2.decode(&buf),
    Encoding::Utf8 => encoding_rs::UTF_8.decode(&buf),
  };
  Ok(encoded_string.into_owned())
}

fn get_results(
  config: &Config,
  crawler: &Box<dyn Crawler>,
) -> Result<Select<Elements<Descendants>>, Error> {
  let mut url = "http://".to_owned();
  url.push_str(&config.host);
  url.push_str(&config.path);

  crawler.log(format!(">> sending request to url '{}' ... ", url));
  let mut response = reqwest::get(url.as_str())?;
  crawler.log(format!("<< received response."));

  crawler.log(format!("parsing document ..."));
  let decoded_response = decode_response(&mut response, &config.encoding)?;
  let document = kuchiki::parse_html()
    .from_utf8()
    .read_from(&mut decoded_response.as_bytes())?;
  crawler.log(format!("document parsed successfully."));

  match document.select(crawler.selector()) {
    Ok(x) => Ok(x),
    Err(()) => Err(Error {
      message: "Main selector did not match.".to_owned(),
    }),
  }
}
