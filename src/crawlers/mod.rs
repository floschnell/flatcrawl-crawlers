mod crawler;
mod immoscout;
mod immowelt;
mod sueddeutsche;
mod wggesucht;

use models::Cities;

pub fn get_crawlers() -> Vec<Box<Crawler>> {
  let mut crawlers: Vec<Box<Crawler>> = Vec::new();

  // immobilienscout24 - München
  crawlers.push(Box::new(ImmoScout::new(
    Cities::Munich,
    "www.immobilienscout24.de",
    "/Suche/S-2/P-1/Wohnung-Miete/Bayern/Muenchen?pagerReporting=true",
  )));

  // immowelt - München
  crawlers.push(Box::new(ImmoWelt::new(
    Cities::Munich,
    "www.immowelt.de",
    "/liste/muenchen/wohnungen/mieten?sort=relevanz",
  )));

  // sueddeutsche - München
  crawlers.push(Box::new(Sueddeutsche::new(
    Cities::Munich,
    "immobilienmarkt.sueddeutsche.de",
    "/Angebote/mieten/Wohnung-Stadt_Muenchen",
  )));

  // wggesucht - München
  crawlers.push(Box::new(WGGesucht::new(
    Cities::Munich,
    "www.wg-gesucht.de",
    "/wohnungen-in-Muenchen.90.2.0.0.html",
  )));

  crawlers
}

pub use crawlers::crawler::Crawler;
pub use crawlers::crawler::Error;
pub use crawlers::immoscout::ImmoScout;
pub use crawlers::immowelt::ImmoWelt;
pub use crawlers::sueddeutsche::Sueddeutsche;
pub use crawlers::wggesucht::WGGesucht;
