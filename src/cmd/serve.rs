use clap::Clap;
use rocket::config::{Config, Environment};

use crate::api;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(short, long, default_value = "8000")]
    port: u16,
}

pub fn run(opts: &Opts) {
    let config = Config::build(Environment::active().unwrap_or(Environment::Development))
        .port(opts.port)
        .finalize()
        .unwrap();
    rocket::custom(config)
        .mount("/config", routes![api::config::index])
        .launch();
}
