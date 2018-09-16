extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Coordinate {
  pub lat: String,
  pub lon: String,
}

#[derive(Debug)]
pub struct Error {
  pub message: String,
}

impl From<reqwest::Error> for Error {
  fn from(_err: reqwest::Error) -> Error {
    return Error {
      message: "Request Error".to_owned(),
    };
  }
}

impl From<url::ParseError> for Error {
  fn from(_err: url::ParseError) -> Error {
    return Error {
      message: "Parse Error".to_owned(),
    };
  }
}

pub fn geocode(nominatim_url: &String, address: &String) -> Result<Coordinate, Error> {
  let mut url = url::Url::parse(nominatim_url)?;
  url.query_pairs_mut().append_pair("q", address.as_str());
  url.query_pairs_mut().append_pair("format", "json");

  let response: Vec<Coordinate> = reqwest::get(url.as_str())?.json()?;

  if response.len() >= 1 {
    Ok(response[0].clone())
  } else {
    Err(Error {
      message: "Not found!".to_owned(),
    })
  }
}
