use chrono::prelude::*;
use models::Cities;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Flat {
  pub source: String,
  pub date: i64,
  pub city: Cities,
  pub data: Option<FlatData>,
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
    }
  }

  pub fn fill(&self, data: &FlatData) -> Flat {
    Flat {
      city: self.city.clone(),
      source: self.source.to_owned(),
      date: self.date,
      data: Some(data.clone()),
    }
  }

  pub fn id(&self) -> Option<String> {
    match self.data {
      Some(ref data) => Some(format!("{}-{}", self.source, data.externalid)),
      None => None,
    }
  }
}
