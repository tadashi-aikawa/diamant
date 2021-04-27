use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::app::trip::TripServiceDb;
use crate::external::gtfs::trips::Trip;
use crate::external::gtfsdb::GtfsDb;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    items: Vec<Trip>,
}

#[get("/<key>/trips?<stop_id>")]
pub fn index(key: String, stop_id: String) -> Json<Response> {
    // TODO: Remove unwrap
    let gtfs = GtfsDb::new(GtfsDb::get_default_path(key).as_path()).unwrap();
    let trips = TripServiceDb::new(gtfs).fetch_trips(stop_id).unwrap();
    Json(Response { items: trips })
}
