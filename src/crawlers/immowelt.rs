extern crate kuchiki;
extern crate regex;
extern crate reqwest;
extern crate std;

use crawlers::{Crawler, Error};
use kuchiki::{ElementData, NodeDataRef};
use models::{Cities, FlatData};

pub struct ImmoWelt {
  pub host: String,
  pub path: String,
  pub city: Cities,
  pub brackets: regex::Regex,
}

impl ImmoWelt {
  pub fn new(city: Cities, host: &'static str, path: &'static str) -> Self {
    return ImmoWelt {
      city,
      host: host.to_owned(),
      path: path.to_owned(),
      brackets: regex::Regex::new(r"\s*\([^)]*\)").unwrap(),
    };
  }
}

impl Crawler for ImmoWelt {
  fn host(&self) -> &String {
    &self.host
  }

  fn path(&self) -> &String {
    &self.path
  }

  fn name(&self) -> &'static str {
    "immowelt"
  }

  fn city(&self) -> &Cities {
    &self.city
  }

  fn selector(&self) -> &'static str {
    ".js-object[data-estateid]"
  }

  fn transform_result(&self, result: NodeDataRef<ElementData>) -> Result<FlatData, Error> {
    let rent = Self::get_text(&result, ".hardfacts_3 .hardfact:nth-child(1) strong")?;
    let squaremeters = Self::get_text(&result, ".hardfacts_3 .hardfact:nth-child(2)")?;
    let rooms = Self::get_text(&result, ".hardfacts_3 .hardfact:nth-child(3)")?;
    let title = Self::get_text(&result, ".listcontent h2")?;
    let address = Self::get_text(&result, ".listlocation")?
      .split("\n")
      .map(|part| part.trim())
      .filter(|part| part.len() > 0)
      .collect::<Vec<_>>()
      .join(", ");
    let cleaned_address = self.brackets.replace_all(&address, "").into_owned();
    let externalid = Self::get_attr(&result, "data-estateid")?;
    Ok(FlatData {
      rent: Self::parse_number(rent)?,
      squaremeters: Self::parse_number(squaremeters)?,
      address: cleaned_address,
      title,
      rooms: Self::parse_number(rooms)?,
      externalid,
    })
  }
}
