extern crate kuchiki;
extern crate regex;
extern crate reqwest;
extern crate std;
extern crate encoding_rs;

use self::regex::Regex;
use kuchiki::iter::*;
use kuchiki::traits::*;
use kuchiki::{ElementData, NodeDataRef};
use models::{City, Flat, FlatData};
use models::Encoding;
use std::ops::Deref;
use reqwest::Response;

#[derive(Debug)]
pub struct Error {
  pub message: String,
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
    return Error {
      message: "Request Error".to_owned(),
    };
  }
}

impl From<std::num::ParseFloatError> for Error {
  fn from(_err: std::num::ParseFloatError) -> Error {
    return Error {
      message: "Could not parse float!".to_owned(),
    };
  }
}

pub trait Crawler: Send + Sync {
  fn name(&self) -> &'static str;
  fn host(&self) -> &String;
  fn path(&self) -> &String;
  fn city(&self) -> &City;
  fn selector(&self) -> &'static str;
  fn transform_result(&self, result: NodeDataRef<ElementData>) -> Result<FlatData, Error>;

  fn encoding(&self) -> &Encoding {
    &Encoding::Utf8
  }

  fn crawl(&self) -> Result<Vec<Flat>, Error> {
    let results = self.get_results()?;
    let mut successful: Vec<Flat> = Vec::new();
    let flat_results: Vec<Result<Flat, Error>> = results
      .map(|result| {
        let flat = Flat::new(self.name().to_owned(), self.city().clone());
        let data = self.transform_result(result)?;
        Ok(flat.fill(&data))
      })
      .collect();
    for flat_result in flat_results {
      match flat_result {
        Ok(flat) => successful.push(flat),
        Err(e) => println!(
          "Could not process flat within crawler '{}', because: {}",
          self.name(),
          e.message
        ),
      }
    }
    Ok(successful)
  }

  fn decode_response(&self, response: &mut Response) -> Result<String, Error> {
    let mut buf: Vec<u8> = vec![];
    response.copy_to(&mut buf)?;
    let (encoded_string, _, _) = match self.encoding() {
      Encoding::Latin1 => encoding_rs::ISO_8859_2.decode(&buf),
      Encoding::Utf8 => encoding_rs::UTF_8.decode(&buf),
    };
    Ok(encoded_string.into_owned())
  }

  fn get_results(&self) -> Result<Select<Elements<Descendants>>, Error> {
    let mut url = "http://".to_owned();
    url.push_str(self.host());
    url.push_str(self.path());

    self.log(format!(">> sending request to url '{}' ... ", url));
    let mut response = reqwest::get(url.as_str())?;
    self.log(format!("<< received response."));

    self.log(format!("parsing document ..."));
    let decoded_response = self.decode_response(&mut response)?;
    let document = kuchiki::parse_html().from_utf8().read_from(&mut decoded_response.as_bytes())?;
    self.log(format!("document parsed successfully."));

    match document.select(self.selector()) {
      Ok(x) => Ok(x),
      Err(()) => Err(Error {
        message: "Main selector did not match.".to_owned(),
      }),
    }
  }

  fn get_attr(element: &NodeDataRef<ElementData>, name: &'static str) -> Result<String, Error>
  where
    Self: Sized,
  {
    match element.deref().attributes.borrow_mut().get(name) {
      Some(val) => Ok(val.to_owned()),
      None => Err(Error {
        message: format!("Could not find attribute '{}'!", name),
      }),
    }
  }

  fn get_text(result: &NodeDataRef<ElementData>, selector: &'static str) -> Result<String, Error>
  where
    Self: Sized,
  {
    match result.as_node().select_first(selector) {
      Ok(el) => Ok(el.text_contents()),
      Err(()) => Err(Error {
        message: format!("Could not find selector '{}'!", selector),
      }),
    }
  }

  fn parse_number(rent_as_str: String) -> Result<f32, Error>
  where
    Self: Sized,
  {
    let rent_regex = Regex::new(r"\d+(\.\d{3})*(,\d+)?").unwrap();
    match rent_regex
      .captures_iter(rent_as_str.as_str())
      .next()
      .and_then(|capture| Some(capture[0].replace(".", "").replace(",", ".")))
    {
      Some(rent) => Ok(rent.parse()?),
      None => Err(Error {
        message: format!("No number found in '{}'!", rent_as_str),
      }),
    }
  }

  fn log(&self, message: String) {
    println!("{}: {}", self.name(), message);
  }
}
