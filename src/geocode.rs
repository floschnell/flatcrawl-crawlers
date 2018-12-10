extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate url;

use std::num::ParseFloatError;
use std::f32;

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResult {
  pub lat: String,
  pub lon: String,
  pub boundingbox: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GeocodeResult {
  pub coord: Coordinate,
  pub uncertainty: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Coordinate {
  pub latitude: f32,
  pub longitude: f32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoundingBox {
  pub min_lat: f32,
  pub max_lat: f32,
  pub min_lon: f32,
  pub max_lon: f32,
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

impl From<ParseFloatError> for Error {
  fn from(_err: ParseFloatError) -> Error {
    return Error {
      message: "Number could not be parsed to float".to_owned(),
    };
  }
}

fn get_distance_from_lat_lon_in_m(lat1: f32, lon1: f32, lat2: f32, lon2: f32) -> f32 {
  let earth_radius_in_m: f32 = 6371000.785;
  let d_lat: f32 = degree_to_radian(lat2-lat1);
  let d_lon: f32 = degree_to_radian(lon2-lon1);
  let a = (d_lat / 2.0).sin() * (d_lat / 2.0).sin() +
      degree_to_radian(lat1).cos() * degree_to_radian(lat2).cos() *
      (d_lon/2.0).sin() * (d_lon/2.0).sin();
  let c = 2.0 * a.sqrt().atan2((1.0 - a).sqrt());
  earth_radius_in_m * c
}

fn degree_to_radian(deg: f32) -> f32 {
  deg * (f32::consts::PI / 180.0)
}

pub fn geocode(nominatim_url: &String, address: &String) -> Result<GeocodeResult, Error> {
  let mut url = url::Url::parse(nominatim_url)?;
  url.query_pairs_mut().append_pair("q", address.as_str());
  url.query_pairs_mut().append_pair("format", "json");

  let response: Vec<ApiResult> = reqwest::get(url.as_str())?.json()?;

  if response.len() >= 1 {
    let best_match: &ApiResult = response.get(0).expect("Results have been empty!");

    let bounds = match (best_match.boundingbox.get(0).map(|c: &String| c.parse::<f32>()),
                        best_match.boundingbox.get(1).map(|c: &String| c.parse::<f32>()),
                        best_match.boundingbox.get(2).map(|c: &String| c.parse::<f32>()),
                        best_match.boundingbox.get(3).map(|c: &String| c.parse::<f32>())) {
      (Some(Ok(min_lat)), Some(Ok(max_lat)), Some(Ok(min_lon)), Some(Ok(max_lon))) =>
        Some(BoundingBox {
          min_lat,
          max_lat,
          min_lon,
          max_lon,
        }),
      _ => None
    };

    let coord = match (best_match.lat.parse::<f32>(), best_match.lon.parse::<f32>()) {
      (Ok(latitude), Ok(longitude)) => Some(Coordinate {
        latitude,
        longitude,
      }),
      _ => None
    };

    match (coord, bounds) {
      (Some(c), Some(b)) => Ok(GeocodeResult {
        coord: c,
        uncertainty: get_distance_from_lat_lon_in_m(b.max_lat, b.max_lon, b.min_lat, b.min_lon),
      }),
      _ => Err(Error {
        message: "Could not geocode location!".to_owned(),
      })
    }
  } else {
    Err(Error {
      message: "Not found!".to_owned(),
    })
  }
}
