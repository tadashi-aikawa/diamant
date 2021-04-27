use std::path::Path;

use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

use crate::api::utils::queries::CommaSeparatedValues;
use crate::app::stop_time::StopTimeServiceDb;
use crate::external;
use crate::external::gtfs::extended::stop_time_details::StopTimeDetail;

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    items: Vec<StopTimeDetail>,
}

#[get("/<key>/stop_time_details?<trip_ids>&<stop_name_prefix>")]
pub fn index(
    key: String,
    trip_ids: Option<CommaSeparatedValues>,
    stop_name_prefix: Option<String>,
) -> Json<Response> {
    // TODO: Remove unwrap
    let gtfs =
        external::gtfsdb::GtfsDb::new(Path::new("db").join(key).join("gtfs.db").as_path()).unwrap();
    let metas = StopTimeServiceDb::new(gtfs)
        .fetch_stop_time_details(trip_ids.map(|x| x.unwrap()), stop_name_prefix)
        .unwrap();
    Json(Response { items: metas })
}
