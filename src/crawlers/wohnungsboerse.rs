extern crate kuchiki;
extern crate reqwest;
extern crate std;

use super::{Crawler, Error};
use crate::models::FlatData;
use kuchiki::{ElementData, NodeDataRef};

impl From<()> for Error {
  fn from(_: ()) -> Self {
    Error {
      message: "".to_owned(),
    }
  }
}

pub struct Wohnungsboerse {}

impl Crawler for Wohnungsboerse {
  fn name(&self) -> &'static str {
    "wohnungsboerse"
  }

  fn selector(&self) -> &'static str {
    ".search_result_entry[class*='estate_']"
  }

  fn transform_result(&self, result: NodeDataRef<ElementData>) -> Result<FlatData, Error> {
    let title = Self::get_text(&result, ".search_result_entry-headline")?
      .trim()
      .to_string();
    let address = Self::get_text(&result, ".search_result_entry-subheadline")?
      .trim()
      .to_string();
    let price = Self::get_attr(
      &result,
      Some("div[itemprop^=priceSpecification] meta[itemprop^=price]"),
      "content",
    )?;
    let squaremeters = Self::get_attr(
      &result,
      Some("div[itemprop^=floorSize] meta[itemprop^=value]"),
      "content",
    )?;
    let rooms = Self::get_attr(
      &result,
      Some("div[itemprop^=numberOfRooms] meta[itemprop^=value]"),
      "content",
    )?;
    let link = Self::get_attr(&result, Some(".search_result_entry-headline a"), "href")?;
    let externalid_opt = link.rsplit("/").next();

    match externalid_opt {
      Some(externalid) => Ok(FlatData {
        rent: Self::parse_number(price)?,
        squaremeters: Self::parse_number(squaremeters)?,
        address,
        title,
        rooms: Self::parse_number(rooms)?,
        externalid: externalid.to_string(),
      }),
      None => Err(Error {
        message: "Could not find an external id".to_string(),
      }),
    }
  }
}
