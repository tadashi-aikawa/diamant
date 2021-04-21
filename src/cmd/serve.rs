use clap::Clap;

use crate::api;

#[derive(Clap, Debug)]
pub struct Opts {}

pub fn run(_opts: &Opts) {
    rocket::ignite()
        .mount("/config", routes![api::config::index])
        .launch();
}
