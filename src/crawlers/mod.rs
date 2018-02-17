mod crawler;
mod immoscout;
mod immowelt;
mod sueddeutsche;
mod wggesucht;

use models::Cities;

pub fn get_crawlers() -> Vec<Box<Crawler>> {
  let mut crawlers: Vec<Box<Crawler>> = Vec::new();

  // Immobilienscout24 ------------------------------------------------
  // München
  crawlers.push(Box::new(ImmoScout::new(
    Cities::Munich,
    "www.immobilienscout24.de",
    "/Suche/S-2/P-1/Wohnung-Miete/Bayern/Muenchen?pagerReporting=true",
  )));

  // Würzburg
  crawlers.push(Box::new(ImmoScout::new(
    Cities::Wuerzburg,
    "www.immobilienscout24.de",
    "/Suche/S-2/P-1/Wohnung-Miete/Bayern/Wuerzburg?pagerReporting=true",
  )));

  // Augsburg
  crawlers.push(Box::new(ImmoScout::new(
    Cities::Augsburg,
    "www.immobilienscout24.de",
    "/Suche/S-2/P-1/Wohnung-Miete/Bayern/Augsburg?pagerReporting=true",
  )));

  // ImmoWelt ---------------------------------------------------------
  // München
  crawlers.push(Box::new(ImmoWelt::new(
    Cities::Munich,
    "www.immowelt.de",
    "/liste/muenchen/wohnungen/mieten?sort=relevanz",
  )));

  // Würzburg
  crawlers.push(Box::new(ImmoWelt::new(
    Cities::Wuerzburg,
    "www.immowelt.de",
    "/liste/wuerzburg/wohnungen/mieten?sort=relevanz",
  )));

  // Augsburg
  crawlers.push(Box::new(ImmoWelt::new(
    Cities::Augsburg,
    "www.immowelt.de",
    "/liste/augsburg/wohnungen/mieten?sort=relevanz",
  )));

  // Süddeutsche ------------------------------------------------------
  // München
  crawlers.push(Box::new(Sueddeutsche::new(
    Cities::Munich,
    "immobilienmarkt.sueddeutsche.de",
    "/Angebote/mieten/Wohnung-Stadt_Muenchen",
  )));

  // Würzburg
  crawlers.push(Box::new(Sueddeutsche::new(
    Cities::Wuerzburg,
    "immobilienmarkt.sueddeutsche.de",
    "/Angebote/mieten/Wohnung-Stadt_Wuerzburg",
  )));

  // WG-Gesucht -------------------------------------------------------
  // München
  crawlers.push(Box::new(WGGesucht::new(
    Cities::Munich,
    "www.wg-gesucht.de",
    "/wohnungen-in-Muenchen.90.2.0.0.html",
  )));

  // Würzburg
  crawlers.push(Box::new(WGGesucht::new(
    Cities::Wuerzburg,
    "www.wg-gesucht.de",
    "/wohnungen-in-Wuerzburg.141.2.0.0.html",
  )));

  // Augsburg
  crawlers.push(Box::new(WGGesucht::new(
    Cities::Augsburg,
    "www.wg-gesucht.de",
    "/wohnungen-in-Augsburg.2.2.0.0.html",
  )));

  crawlers
}

pub use crawlers::crawler::Crawler;
pub use crawlers::crawler::Error;
pub use crawlers::immoscout::ImmoScout;
pub use crawlers::immowelt::ImmoWelt;
pub use crawlers::sueddeutsche::Sueddeutsche;
pub use crawlers::wggesucht::WGGesucht;
