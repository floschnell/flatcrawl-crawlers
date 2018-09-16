use chrono::prelude::*;
use geocode::Coordinate;
use models::Cities;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
  pub latitude: f32,
  pub longitude: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Flat {
  pub source: String,
  pub date: i64,
  pub city: Cities,
  pub data: Option<FlatData>,
  pub location: Option<Location>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlatData {
  pub rent: f32,
  pub squaremeters: f32,
  pub address: String,
  pub title: String,
  pub externalid: String,
  pub rooms: f32,
}

impl Flat {
  pub fn new(source: String, city: Cities) -> Flat {
    Flat {
      date: Utc::now().timestamp(),
      source,
      data: None,
      city,
      location: None,
    }
  }

  pub fn fill(&self, data: &FlatData) -> Flat {
    Flat {
      city: self.city.clone(),
      source: self.source.to_owned(),
      date: self.date,
      data: Some(data.clone()),
      location: self.location.clone(),
    }
  }

  pub fn locate(&self, coord: &Coordinate) -> Flat {
    Flat {
      city: self.city.clone(),
      source: self.source.to_owned(),
      date: self.date,
      data: self.data.clone(),
      location: match (coord.lat.parse::<f32>(), coord.lon.parse::<f32>()) {
        (Ok(latitude), Ok(longitude)) => Some(Location {
          latitude,
          longitude,
        }),
        _ => None,
      },
    }
  }

  pub fn id(&self) -> Option<String> {
    match self.data {
      Some(ref data) => Some(format!("{}-{}", self.source, data.externalid)),
      None => None,
    }
  }
}
