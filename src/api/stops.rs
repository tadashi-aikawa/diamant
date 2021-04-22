use crate::app::trip::TripServiceDb;
use crate::external;
use crate::external::gtfs::extended::trip_with_sequence_meta::TripWithSequenceMeta;
use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    items: Vec<TripWithSequenceMeta>,
}

#[get("/?<trip_id>")]
pub fn index(trip_id: String) -> Json<Response> {
    // TODO: Remove unwrap
    let gtfs = external::gtfsdb::GtfsDb::new(Path::new("diamant.db")).unwrap();
    let metas = TripServiceDb::new(gtfs)
        .fetch_trip_with_sequence_metas(Some(trip_id))
        .unwrap();
    Json(Response { items: metas })
}
