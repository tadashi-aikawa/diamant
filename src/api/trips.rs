use std::path::Path;

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::app::trip::TripServiceDb;
use crate::external;
use crate::external::gtfs::trips::Trip;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    items: Vec<Trip>,
}

#[get("/<key>/trips?<stop_id>")]
pub fn index(key: String, stop_id: String) -> Json<Response> {
    // TODO: Remove unwrap
    let gtfs =
        external::gtfsdb::GtfsDb::new(Path::new("db").join(key).join("gtfs.db").as_path()).unwrap();
    let trips = TripServiceDb::new(gtfs).fetch_trips(stop_id).unwrap();
    Json(Response { items: trips })
}
