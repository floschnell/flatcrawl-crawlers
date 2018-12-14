use models::City;
use models::Encoding;
use crawlers::CrawlerImpl;

pub struct Config {
    pub host: &'static str,
    pub path: &'static str,
    pub city: City,
    pub encoding: Encoding,
    pub crawler: CrawlerImpl,
}