mod config;
mod crawler;
mod executor;
mod immoscout;
mod immowelt;
mod sueddeutsche;
mod wggesucht;
mod wohnungsboerse;

use crate::models::City;
use crate::models::Encoding;
use crate::models::ContractType;
use crate::models::PropertyType;

pub use crate::crawlers::config::Config;
pub use crate::crawlers::crawler::Crawler;
pub use crate::crawlers::crawler::Error;
pub use crate::crawlers::executor::execute;
pub use crate::crawlers::immoscout::ImmoScout;
pub use crate::crawlers::immowelt::ImmoWelt;
pub use crate::crawlers::sueddeutsche::Sueddeutsche;
pub use crate::crawlers::wggesucht::WGGesucht;
pub use crate::crawlers::wohnungsboerse::Wohnungsboerse;

pub enum CrawlerImpl {
  ImmoScoutRentApartments,
  ImmoScoutBuyHouses,
  ImmoWelt,
  Sueddeutsche,
  WGGesucht,
  Wohnungsboerse,
}

pub fn get_crawler(crawler_impl: &CrawlerImpl) -> Result<Box<dyn Crawler>, Error> {
  match crawler_impl {
    CrawlerImpl::ImmoWelt => Ok(Box::new(ImmoWelt::new())),
    CrawlerImpl::WGGesucht => Ok(Box::new(WGGesucht {})),
    CrawlerImpl::Sueddeutsche => Ok(Box::new(Sueddeutsche::new())),
    CrawlerImpl::ImmoScoutRentApartments => Ok(Box::new(ImmoScout { contract_type: ContractType::Rent, property_type: PropertyType::Flat })),
    CrawlerImpl::ImmoScoutBuyHouses => Ok(Box::new(ImmoScout { contract_type: ContractType::Buy, property_type: PropertyType::House })),
    CrawlerImpl::Wohnungsboerse => Ok(Box::new(Wohnungsboerse {})),
  }
}

pub fn get_crawler_configs() -> Vec<Config> {
  let mut configs: Vec<Config> = Vec::new();

  // Immobilienscout24 ------------------------------------------------

  // Lindenberg
  configs.push(Config {
    city: City::Lindenberg,
    host: "www.immobilienscout24.de",
    path: "/Suche/de/bayern/lindau-bodensee-kreis/lindenberg-im-allgaeu/haus-kaufen",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::ImmoScoutBuyHouses,
  });

  // München
  configs.push(Config {
    city: City::Munich,
    host: "www.immobilienscout24.de",
    path: "/Suche/S-2/P-1/Wohnung-Miete/Bayern/Muenchen?pagerReporting=true",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::ImmoScoutRentApartments,
  });

  // Würzburg
  configs.push(Config {
    city: City::Wuerzburg,
    host: "www.immobilienscout24.de",
    path: "/Suche/S-2/P-1/Wohnung-Miete/Bayern/Wuerzburg?pagerReporting=true",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::ImmoScoutRentApartments,
  });

  // Augsburg
  configs.push(Config {
    city: City::Augsburg,
    host: "www.immobilienscout24.de",
    path: "/Suche/S-2/P-1/Wohnung-Miete/Bayern/Augsburg?pagerReporting=true",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::ImmoScoutRentApartments,
  });

  // Kempten
  configs.push(Config {
    city: City::Kempten,
    host: "www.immobilienscout24.de",
    path: "/Suche/S-2/P-1/Wohnung-Miete/Bayern/Kempten-Allgaeu?pagerReporting=true",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::ImmoScoutRentApartments,
  });

  // ImmoWelt ------------------------------------------------
  // München
  configs.push(Config {
    city: City::Munich,
    host: "www.immowelt.de",
    path: "/liste/muenchen/wohnungen/mieten?sort=relevanz",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::ImmoWelt,
  });

  // Würzburg
  configs.push(Config {
    city: City::Wuerzburg,
    host: "www.immowelt.de",
    path: "/liste/wuerzburg/wohnungen/mieten?sort=relevanz",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::ImmoWelt,
  });

  // Augsburg
  configs.push(Config {
    city: City::Augsburg,
    host: "www.immowelt.de",
    path: "/liste/augsburg/wohnungen/mieten?sort=relevanz",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::ImmoWelt,
  });

  // Kempten
  configs.push(Config {
    city: City::Kempten,
    host: "www.immowelt.de",
    path: "/liste/kempten-allgaeu/wohnungen/mieten?sort=relevanz",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::ImmoWelt,
  });

  // Süddeutsche ------------------------------------------------
  // München
  configs.push(Config {
    city: City::Munich,
    host: "immobilienmarkt.sueddeutsche.de",
    path: "/Angebote/mieten/Wohnung-Stadt_Muenchen",
    encoding: Encoding::Latin1,
    crawler: CrawlerImpl::Sueddeutsche,
  });

  // Würzburg
  configs.push(Config {
    city: City::Wuerzburg,
    host: "immobilienmarkt.sueddeutsche.de",
    path: "/Angebote/mieten/Wohnung-Stadt_Wuerzburg",
    encoding: Encoding::Latin1,
    crawler: CrawlerImpl::Sueddeutsche,
  });

  // WG-Gesucht -------------------------------------------------------
  // München
  configs.push(Config {
    city: City::Munich,
    host: "www.wg-gesucht.de",
    path: "/wohnungen-in-Muenchen.90.2.0.0.html",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::WGGesucht,
  });

  // Würzburg
  configs.push(Config {
    city: City::Wuerzburg,
    host: "www.wg-gesucht.de",
    path: "/wohnungen-in-Wuerzburg.141.2.0.0.html",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::WGGesucht,
  });

  // Augsburg
  configs.push(Config {
    city: City::Augsburg,
    host: "www.wg-gesucht.de",
    path: "/wohnungen-in-Augsburg.2.2.0.0.html",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::WGGesucht,
  });

  // Kempten
  configs.push(Config {
    city: City::Kempten,
    host: "www.wg-gesucht.de",
    path: "/wohnungen-in-Kempten-Allgaeu.70.2.0.0.html",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::WGGesucht,
  });

  // Wohnungsboerse -------------------------------------------------------
  // München
  configs.push(Config {
    city: City::Munich,
    host: "www.wohnungsboerse.net",
    path: "/searches/index/marketing_type:miete/object_type:1/country:de/minrooms:1/state:2/cities:2091",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::Wohnungsboerse,
  });

  // Würzburg
  configs.push(Config {
    city: City::Wuerzburg,
    host: "www.wohnungsboerse.net",
    path: "/searches/index/marketing_type:miete/object_type:1/country:de/minrooms:1/state:2/cities:2772",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::Wohnungsboerse,
  });

  // Augsburg
  configs.push(Config {
    city: City::Augsburg,
    host: "www.wohnungsboerse.net",
    path: "/searches/index/marketing_type:miete/object_type:1/country:de/minrooms:1/state:2/cities:1231",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::Wohnungsboerse,
  });

  // Kempten
  configs.push(Config {
    city: City::Kempten,
    host: "www.wohnungsboerse.net",
    path: "/searches/index/marketing_type:miete/object_type:1/country:de/minrooms:1/state:2/cities:1879",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::Wohnungsboerse,
  });

  configs
}
