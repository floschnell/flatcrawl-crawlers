use crate::geocode::Coordinate;
use crate::models::city::City;
use chrono::prelude::*;
use regex::Regex;
use serde_derive::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Location {
  pub latitude: f32,
  pub longitude: f32,
  pub uncertainty: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Flat {
  pub source: String,
  pub date: i64,
  pub city: City,
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

impl PartialEq for Flat {
  fn eq(&self, other: &Self) -> bool {
    self.is_equal_to(other)
      || (self.city == other.city
        && self.source == other.source
        && self.data.is_some()
        && other.data.is_some()
        && self.data.as_ref().unwrap().externalid == other.data.as_ref().unwrap().externalid)
  }
}

impl Flat {
  fn is_equal_to(&self, other: &Self) -> bool {
    let special_characters_regex = Regex::new("[^0-9a-zA-Z]+").unwrap();
    self.city == other.city
      && match (&self.data, &other.data) {
        (None, None) => false,
        (Some(ref d1), Some(ref d2)) => {
          special_characters_regex.replace_all(&d1.title.to_lowercase(), "")
            == special_characters_regex.replace_all(&d2.title.to_lowercase(), "")
        }
        _ => false,
      }
  }

  pub fn new(source: String, city: City) -> Flat {
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

  pub fn locate(&self, coord: &Coordinate, uncertainty: f32) -> Flat {
    Flat {
      city: self.city.clone(),
      source: self.source.to_owned(),
      date: self.date,
      data: self.data.clone(),
      location: Some(Location {
        latitude: coord.latitude,
        longitude: coord.longitude,
        uncertainty,
      }),
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::models::City;
  use crate::models::Flat;
  use crate::models::FlatData;

  #[test]
  fn compare_flat_simple() {
    let flat = Flat::new(String::from("some source"), City::Munich);
    let another_flat = Flat::new(String::from("some source"), City::Munich);
    assert_eq!(flat, another_flat);
  }

  #[test]
  fn compare_flat_complex() {
    let flat_a = Flat {
      source: String::from("some source A"),
      city: City::Munich,
      date: 0,
      data: Some(FlatData {
        rent: 100.,
        squaremeters: 100.,
        address: String::from("Some address"),
        title: String::from("This is some title"),
        externalid: String::from("1a"),
        rooms: 3.,
      }),
      location: None,
    };

    let flat_b = Flat {
      source: String::from("some source B"),
      city: City::Munich,
      date: 0,
      data: Some(FlatData {
        rent: 100.,
        squaremeters: 100.,
        address: String::from("Some address"),
        title: String::from("This is some title"),
        externalid: String::from("1b"),
        rooms: 3.,
      }),
      location: None,
    };

    assert_eq!(flat_a, flat_b);
  }

  #[test]
  fn compare_flat_complex_special_chars() {
    let flat_a = Flat {
      source: String::from("some source A"),
      city: City::Munich,
      date: 0,
      data: Some(FlatData {
        rent: 100.,
        squaremeters: 100.,
        address: String::from("Some address"),
        title: String::from("This is% some title!"),
        externalid: String::from("1a"),
        rooms: 3.,
      }),
      location: None,
    };

    let flat_b = Flat {
      source: String::from("some source B"),
      city: City::Munich,
      date: 0,
      data: Some(FlatData {
        rent: 101.,
        squaremeters: 101.,
        address: String::from("Some other address"),
        title: String::from("This is some title"),
        externalid: String::from("1b"),
        rooms: 3.5,
      }),
      location: None,
    };

    assert_eq!(flat_a, flat_b);
  }

  #[test]
  fn compare_flat_not_equal() {
    let flat_a = Flat {
      source: String::from("some source A"),
      city: City::Munich,
      date: 0,
      data: None,
      location: None,
    };

    let flat_b = Flat {
      source: String::from("some source B"),
      city: City::Munich,
      date: 0,
      data: None,
      location: None,
    };

    assert_ne!(flat_a, flat_b);
  }

  #[test]
  fn compare_flat_complex_not_equal() {
    let flat_a = Flat {
      source: String::from("some source A"),
      city: City::Munich,
      date: 0,
      data: Some(FlatData {
        rent: 100.,
        squaremeters: 100.,
        address: String::from("Some address"),
        title: String::from("This is% some title!"),
        externalid: String::from("1a"),
        rooms: 3.,
      }),
      location: None,
    };

    let flat_b = Flat {
      source: String::from("some source B"),
      city: City::Munich,
      date: 0,
      data: Some(FlatData {
        rent: 101.,
        squaremeters: 101.,
        address: String::from("Some other address"),
        title: String::from("This is some other title"),
        externalid: String::from("1b"),
        rooms: 3.5,
      }),
      location: None,
    };

    assert_ne!(flat_a, flat_b);
  }
}
