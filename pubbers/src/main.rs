use config::{Config, File};
use flexi_logger::Logger;

#[path = "config.rs"] // avoid mod name conflict
mod app_config;
mod redis_blurbs;
mod rabbitmq_blurbs;

use crate::app_config::AppConfig;

fn main() {
    // parse app config
    let mut config = Config::default();
    config.merge(File::with_name("resources/dev_config.toml")).unwrap();
    let app_config = config.try_into::<AppConfig>().unwrap();

    // setup and start logging
    let log_level: &String = &app_config.logging.log_level;
    Logger::try_with_env_or_str(log_level).unwrap()
        .log_to_stdout()
        .start().unwrap();

    redis_blurbs::redis_blurb_01();
    //rabbitmq_blurbs::rabbitmq_blurb_01();

    println!("Hello, world!");
}
