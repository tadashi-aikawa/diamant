use std::path::Path;

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::app::stops::StopServiceDb;
use crate::external;
use crate::external::gtfs::stops::Stop;
use crate::external::gtfsdb::GtfsDb;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    items: Vec<Stop>,
}

#[get("/<key>/stops?<word>")]
pub fn index(key: String, word: String) -> Json<Response> {
    // TODO: Remove unwrap
    let gtfs = GtfsDb::new(GtfsDb::get_default_path(key).as_path()).unwrap();
    let stops = StopServiceDb::new(gtfs).fetch_stops(word).unwrap();
    Json(Response { items: stops })
}
