use std::path::Path;

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::app::stops::StopServiceDb;
use crate::external;
use crate::external::gtfs::stops::Stop;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    items: Vec<Stop>,
}

#[get("/<key>/stops?<word>")]
pub fn index(key: String, word: String) -> Json<Response> {
    // TODO: Remove unwrap
    let gtfs =
        external::gtfsdb::GtfsDb::new(Path::new("db").join(key).join("gtfs.db").as_path()).unwrap();
    let stops = StopServiceDb::new(gtfs).fetch_stops(word).unwrap();
    Json(Response { items: stops })
}
