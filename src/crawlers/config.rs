use super::CrawlerImpl;
use crate::models::City;
use crate::models::Encoding;

pub struct Config {
  pub host: &'static str,
  pub path: &'static str,
  pub city: City,
  pub encoding: Encoding,
  pub crawler: CrawlerImpl,
}
