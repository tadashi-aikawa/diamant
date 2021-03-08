use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::external::gtfs::agency::Agency;
use crate::external::gtfs::calendar::Calendar;
use crate::external::gtfs::routes::{Route, RouteId};
use crate::external::gtfs::stop_times::StopTime;
use crate::external::gtfs::stops::Stop;
use crate::external::gtfs::trips::Trip;

pub mod agency;
pub mod calendar;
pub mod routes;
pub mod stop_times;
pub mod stops;
pub mod trips;

/// 色. 00FFFF など 6 桁の 16 進数
pub type Color = String;
/// メートル
pub type Meter = u32;
/// 順序 (ex: 0)
pub type Sequence = u32;
/// 電話番号 (ex: 03-2816-5700)
pub type TelephoneNumber = String;
/// メールアドレス
pub type MailAddress = String;
/// Url
pub type Url = String;

// TODO (ex: Z_210)
pub type ZoneId = String;

#[derive(Debug, Serialize, Deserialize)]
pub enum Timezone {
    /// 日本語
    #[serde(rename = "Asia/Tokyo")]
    AsiaTokyo,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    /// 日本語
    Ja,
}

pub trait Gtfs {
    fn create_all(&self) -> Result<()>;
    fn drop_all(&self) -> Result<()>;
    fn insert_agencies(&mut self, agencies: &[Agency]) -> Result<()>;
    fn select_agencies(&mut self) -> Result<Vec<Agency>>;
    fn insert_stops(&mut self, stops: &[Stop]) -> Result<()>;
    fn select_stops(&mut self) -> Result<Vec<Stop>>;
    fn insert_routes(&mut self, routes: &[Route]) -> Result<()>;
    fn select_routes(&mut self, route_id: &Option<RouteId>) -> Result<Vec<Route>>;
    fn insert_trips(&mut self, trips: &[Trip]) -> Result<()>;
    fn select_trips(&mut self, route_id: &Option<RouteId>) -> Result<Vec<Trip>>;
    fn insert_stop_times(&mut self, stop_times: &[StopTime]) -> Result<()>;
    fn insert_calendars(&mut self, calendars: &[Calendar]) -> Result<()>;
    fn select_calendars(&mut self) -> Result<Vec<Calendar>>;
}
