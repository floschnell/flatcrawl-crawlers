use config::{Config, File};

pub struct AmqpConfig {
  pub host: String,
  pub queue: String,
  pub username: String,
  pub password: String,
}

pub struct CrawlerConfig {
  pub thread_count: i32,
  pub amqp_config: AmqpConfig,
}

pub fn read() -> CrawlerConfig {
  let mut config = Config::new();
  config.merge(File::with_name("config")).unwrap();
  let host = config.get("amqp.host").unwrap();
  let queue = config.get("amqp.queue").unwrap();
  let username = config.get("amqp.username").unwrap();
  let password = config.get("amqp.password").unwrap();
  let thread_count: String = config.get("thread_count").unwrap();

  CrawlerConfig {
    thread_count: thread_count.parse().unwrap(),
    amqp_config: AmqpConfig {
      host,
      queue,
      username,
      password,
    },
  }
}
