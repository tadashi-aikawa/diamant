use rocket_contrib::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct Response {
    version: String,
}

#[get("/")]
pub fn index() -> Json<Response> {
    Json(Response {
        version: clap::crate_version!().into(),
    })
}
