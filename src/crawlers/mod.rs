mod crawler;
mod immoscout;
mod immowelt;
mod sueddeutsche;
mod wggesucht;
mod config;
mod executor;

use models::City;
use models::Encoding;

pub use crawlers::crawler::Crawler;
pub use crawlers::crawler::Error;
pub use crawlers::immoscout::ImmoScout;
pub use crawlers::immowelt::ImmoWelt;
pub use crawlers::sueddeutsche::Sueddeutsche;
pub use crawlers::wggesucht::WGGesucht;
pub use crawlers::config::Config;
pub use crawlers::executor::execute;

pub enum CrawlerImpl {
  ImmoScout,
  ImmoWelt,
  Sueddeutsche,
  WGGesucht,
}

pub fn get_crawler(crawler_impl: &CrawlerImpl) -> Result<Box<Crawler>, Error> {
  match crawler_impl {
    CrawlerImpl::ImmoWelt => Ok(Box::new(ImmoWelt::new())),
    CrawlerImpl::WGGesucht => Ok(Box::new(WGGesucht {})),
    CrawlerImpl::Sueddeutsche => Ok(Box::new(Sueddeutsche::new())),
    CrawlerImpl::ImmoScout => Ok(Box::new(ImmoScout {})),
  }
}

pub fn get_crawler_configs() -> Vec<Config> {
  let mut configs: Vec<Config> = Vec::new();

  // Immobilienscout24 ------------------------------------------------
  // München
  configs.push(Config {
    city: City::Munich,
    host: "www.immobilienscout24.de",
    path: "/Suche/S-2/P-1/Wohnung-Miete/Bayern/Muenchen?pagerReporting=true",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::ImmoScout,
  });

  // Würzburg
  configs.push(Config {
    city: City::Wuerzburg,
    host: "www.immobilienscout24.de",
    path: "/Suche/S-2/P-1/Wohnung-Miete/Bayern/Wuerzburg?pagerReporting=true",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::ImmoScout,
  });

  // Augsburg
  configs.push(Config {
    city: City::Augsburg,
    host: "www.immobilienscout24.de",
    path: "/Suche/S-2/P-1/Wohnung-Miete/Bayern/Augsburg?pagerReporting=true",
    encoding: Encoding::Utf8,
    crawler: CrawlerImpl::ImmoScout,
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

  configs
}

