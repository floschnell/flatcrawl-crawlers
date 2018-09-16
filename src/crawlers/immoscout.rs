extern crate kuchiki;
extern crate reqwest;
extern crate std;

use crawlers::{Crawler, Error};
use kuchiki::{ElementData, NodeDataRef};
use models::{Cities, FlatData};

pub struct ImmoScout {
  pub host: String,
  pub path: String,
  pub city: Cities,
}

impl ImmoScout {
  pub fn new(city: Cities, host: &'static str, path: &'static str) -> Self {
    return ImmoScout {
      city,
      host: host.to_owned(),
      path: path.to_owned(),
    };
  }
}

impl Crawler for ImmoScout {
  fn host(&self) -> &String {
    &self.host
  }

  fn path(&self) -> &String {
    &self.path
  }

  fn name(&self) -> &'static str {
    "immoscout"
  }

  fn city(&self) -> &Cities {
    &self.city
  }

  fn selector(&self) -> &'static str {
    "article[data-item=result]"
  }

  fn transform_result(&self, result: NodeDataRef<ElementData>) -> Result<FlatData, Error> {
    let rent = Self::get_text(&result, ".result-list-entry__criteria dl:nth-child(1) dd")?;
    let squaremeters = Self::get_text(&result, ".result-list-entry__criteria dl:nth-child(2) dd")?;
    let rooms = Self::get_text(
      &result,
      ".result-list-entry__criteria dl:nth-child(3) dd .onlyLarge",
    )?;
    let title = Self::get_text(&result, ".result-list-entry__brand-title")?;
    let address = Self::get_text(&result, ".result-list-entry__map-link div")?;
    let externalid = Self::get_attr(&result, "data-obid")?.trim().to_owned();
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
