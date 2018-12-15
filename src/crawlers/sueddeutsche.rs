extern crate kuchiki;
extern crate regex;
extern crate reqwest;
extern crate std;

use crawlers::{Crawler, Error};
use kuchiki::{ElementData, NodeDataRef};
use models::{FlatData};
use std::ops::Deref;

pub struct Sueddeutsche {
  pub brackets: regex::Regex,
}

impl Sueddeutsche {
  pub fn new() -> Self {
    return Sueddeutsche {
      brackets: regex::Regex::new(r"\s*\([^)]*\)").unwrap(),
    };
  }
}

impl Crawler for Sueddeutsche {

  fn name(&self) -> &'static str {
    "sueddeutsche"
  }

  fn selector(&self) -> &'static str {
    "#idHitContent .hitRow"
  }

  fn transform_result<'a>(&self, result: NodeDataRef<ElementData>) -> Result<FlatData, Error> {
    let hit_rooms_div_text = Self::get_text(&result, ".hitRoomsDiv")?;
    let hit_rooms_div_elements: Vec<&str> = hit_rooms_div_text.split(", ").collect();
    let squaremeters_opt: Option<&&str> = hit_rooms_div_elements.get(0);
    let rooms_opt: Option<&&str> = hit_rooms_div_elements.get(1);

    let hit_regions_text = Self::get_text(&result, ".hitRegionTxt")?.replace("\t", "");
    let hit_regions_elements: Vec<&str> = hit_regions_text.split("\n").collect();
    let address_opt = hit_regions_elements.get(2);

    let title = Self::get_text(&result, ".hitHeadline")?
      .replace("\t", "")
      .replace("\n", "");

    let rent = Self::get_text(&result, ".hitPrice")?.replace("&nbsp;", " ");

    let externalid = Self::get_attr(&result, None, "id")?.replace("idHitRowList", "");

    match (&squaremeters_opt, &rooms_opt, &address_opt) {
      (&Some(squaremeters), &Some(rooms), &Some(address)) => Ok(FlatData {
        rent: Self::parse_number(rent)?,
        squaremeters: Self::parse_number(squaremeters.deref().to_owned())?,
        address: self.brackets.replace_all(address.deref(), "").into_owned(),
        title,
        rooms: Self::parse_number(rooms.deref().to_owned())?,
        externalid,
      }),
      _ => Err(Error {
        message: format!(
          "Information is incomplete: {:?}, {:?}, {:?}!",
          squaremeters_opt, rooms_opt, address_opt
        ),
      }),
    }
  }
}
