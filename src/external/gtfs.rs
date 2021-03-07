use anyhow::Result;

use crate::external::gtfs::routes::{Route, RouteId};
use crate::external::gtfs::stop_times::StopTime;
use crate::external::gtfs::trips::Trip;

pub mod routes;
pub mod stop_times;
pub mod trips;

/// GTFS-JPで使う色. 00FFFF など 6 桁の 16 進数
pub type Color = String;
/// メートル
pub type Meter = u32;
/// 順序
pub type Sequence = u32;

/// TODO
pub type StopId = String;

pub trait Gtfs {
    fn create_all(&self) -> Result<()>;
    fn drop_all(&self) -> Result<()>;
    fn insert_routes(&mut self, routes: &[Route]) -> Result<()>;
    fn select_routes(&mut self, route_id: &Option<RouteId>) -> Result<Vec<Route>>;
    fn insert_stop_times(&mut self, stop_times: &[StopTime]) -> Result<()>;
    fn insert_trips(&mut self, trips: &[Trip]) -> Result<()>;
    fn select_trips(&mut self, route_id: &Option<RouteId>) -> Result<Vec<Trip>>;
}
