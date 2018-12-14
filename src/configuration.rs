use config::{Config, File};

#[derive(Clone, Debug)]
pub struct AmqpConfig {
  pub host: String,
  pub queue: String,
  pub username: String,
  pub password: String,
}

#[derive(Clone, Debug)]
pub struct ApplicationConfig {
  pub test: bool,
  pub thread_count: i32,
  pub nominatim_url: String,
  pub amqp_config: AmqpConfig,
}

pub fn read() -> ApplicationConfig {
  let mut config = Config::new();
  config.merge(File::with_name("config")).unwrap();
  let test = config.get("test").unwrap();
  let host = config.get("amqp.host").unwrap();
  let queue = config.get("amqp.queue").unwrap();
  let username = config.get("amqp.username").unwrap();
  let password = config.get("amqp.password").unwrap();
  let thread_count: String = config.get("thread_count").unwrap();
  let nominatim_url: String = config.get("nominatim_url").unwrap();

  ApplicationConfig {
    test,
    thread_count: thread_count.parse().unwrap(),
    nominatim_url,
    amqp_config: AmqpConfig {
      host,
      queue,
      username,
      password,
    },
  }
}
