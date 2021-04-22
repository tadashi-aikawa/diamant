use clap::Clap;
use rocket::config::{Config, Environment};
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use rocket::{Request, Response};

use crate::api;

pub struct CORS;

impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, GET, PATCH, OPTIONS",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(short, long, default_value = "8000")]
    port: u16,
    #[clap(long)]
    cors: bool,
}

pub fn run(opts: &Opts) {
    let config = Config::build(Environment::active().unwrap_or(Environment::Development))
        .port(opts.port)
        .finalize()
        .unwrap();

    let mut app = rocket::custom(config)
        .mount("/config", routes![api::config::index])
        .mount("/stops", routes![api::stops::index]);

    if opts.cors {
        app = app.attach(CORS);
    }

    app.launch();
}
