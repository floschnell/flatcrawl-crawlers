extern crate kuchiki;
extern crate reqwest;
extern crate std;

use crawlers::{Crawler, Error};
use kuchiki::{ElementData, NodeDataRef};
use models::{Cities, FlatData};

pub struct Sueddeutsche {
  pub host: String,
  pub path: String,
  pub city: Cities,
}

impl Sueddeutsche {
  pub fn new(city: Cities, host: &'static str, path: &'static str) -> Self {
    return Sueddeutsche {
      city,
      host: host.to_owned(),
      path: path.to_owned(),
    };
  }
}

impl Crawler for Sueddeutsche {
  fn host(&self) -> &String {
    &self.host
  }

  fn path(&self) -> &String {
    &self.path
  }

  fn name(&self) -> &'static str {
    "sueddeutsche"
  }

  fn city(&self) -> &Cities {
    &self.city
  }

  fn selector(&self) -> &'static str {
    "#idHitContent .hitRow"
  }

  fn transform_result(&self, result: NodeDataRef<ElementData>) -> Result<FlatData, Error> {
    let rent = Self::get_text(&result, ".hitPrice")?
      .replace("&nbsp;", " ");
    let squaremeters = Self::get_text(&result, ".hitRoomsDiv")?
      .split(", ")
      .collect::<Vec<_>>()[0].to_owned();
    let rooms = Self::get_text(&result, ".hitRoomsDiv")?
      .split(", ")
      .collect::<Vec<_>>()[1].to_owned();
    let title = Self::get_text(&result, ".hitHeadline")?
      .replace("\t", "")
      .replace("\n", "");
    let address = Self::get_text(&result, ".hitRegionTxt")?
      .replace("\t", "")
      .split("\n")
      .collect::<Vec<_>>()[2].to_owned();
    let externalid = Self::get_attr(&result, "id")?
      .replace("idHitRowList", "");
    Ok(FlatData {
      rent: Self::parse_number(rent)?,
      squaremeters: Self::parse_number(squaremeters)?,
      address,
      title,
      rooms: Self::parse_number(rooms)?,
      externalid,
    })
  }
}
