extern crate kuchiki;
extern crate reqwest;
extern crate std;

use crawlers::{Crawler, Error};
use kuchiki::{ElementData, NodeDataRef};
use models::{Cities, FlatData};

pub struct WGGesucht {
  pub host: String,
  pub path: String,
  pub city: Cities,
}

impl WGGesucht {
  pub fn new(city: Cities, host: &'static str, path: &'static str) -> Self {
    return WGGesucht {
      city,
      host: host.to_owned(),
      path: path.to_owned(),
    };
  }
}

impl Crawler for WGGesucht {
  fn host(&self) -> &String {
    &self.host
  }

  fn path(&self) -> &String {
    &self.path
  }

  fn name(&self) -> &'static str {
    "wggesucht"
  }

  fn city(&self) -> &Cities {
    &self.city
  }

  fn selector(&self) -> &'static str {
    "tr[adid^=wohnungen]"
  }

  fn transform_result(&self, result: NodeDataRef<ElementData>) -> Result<FlatData, Error> {
    let only_limited = Self::get_text(&result, ".ang_spalte_freibis")?
      .trim()
      .len() > 0;
    if only_limited {
      Err( Error { message: "Flat is only available for a limited time.".to_owned() } )
    } else {
      let rent = Self::get_text(&result, ".ang_spalte_miete")?;
      let squaremeters = Self::get_text(&result, ".ang_spalte_groesse")?;
      let rooms = Self::get_text(&result, ".ang_spalte_zimmer")?;
      let title = "Wohnung auf WG Gesucht".to_owned();
      let address = "München, ".to_owned() +
        Self::get_text(&result, ".ang_spalte_stadt")?
        .replace("\n", "")
        .trim();
      let externalid = Self::get_attr(&result, "adid")?;
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
}
