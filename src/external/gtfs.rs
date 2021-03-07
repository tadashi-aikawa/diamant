use anyhow::Result;

use crate::external::gtfs::routes::Route;
use crate::external::gtfs::stop_times::StopTime;
use crate::external::gtfs::trips::{RouteId, Trip};

pub mod routes;
pub mod stop_times;
pub mod trips;

pub trait Gtfs {
    fn create_all(&self) -> Result<()>;
    fn drop_all(&self) -> Result<()>;
    fn insert_routes(&mut self, routes: &[Route]) -> Result<()>;
    fn insert_stop_times(&mut self, stop_times: &[StopTime]) -> Result<()>;
    fn insert_trips(&mut self, trips: &[Trip]) -> Result<()>;
    fn select_trips_by_route_id(&mut self, route_id: &RouteId) -> Result<Vec<Trip>>;
}
