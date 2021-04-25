use crate::api::utils::queries::CommaSeparatedValues;
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

#[get("/<key>/stops?<trip_ids>")]
pub fn index(key: String, trip_ids: CommaSeparatedValues) -> Json<Response> {
    // TODO: Remove unwrap
    let gtfs =
        external::gtfsdb::GtfsDb::new(Path::new("db").join(key).join("gtfs.db").as_path()).unwrap();
    let metas = TripServiceDb::new(gtfs)
        .fetch_trip_with_sequence_metas(Some(trip_ids.unwrap()))
        .unwrap();
    Json(Response { items: metas })
}
