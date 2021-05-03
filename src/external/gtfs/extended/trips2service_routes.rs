use serde::{Deserialize, Serialize};

pub use crate::external::gtfs::trips::{Trip, TripId};

use crate::external::gtfs::extended::service_routes::ServiceRouteId;
use crate::external::gtfsdb::Table;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct Trip2ServiceRoute {
    /// 便ID
    pub trip_id: TripId,
    /// サービスルートID
    pub service_route_id: ServiceRouteId,
}

impl Table for Trip2ServiceRoute {
    fn table_name() -> &'static str {
        "trips2service_routes"
    }

    fn column_names() -> &'static [&'static str] {
        &["trip_id", "service_route_id"]
    }

    fn create_sql() -> &'static str {
        "
        trip_id text,
        service_route_id int,
        PRIMARY KEY(trip_id, service_route_id)
        "
    }
}
