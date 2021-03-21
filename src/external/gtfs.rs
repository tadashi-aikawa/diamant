use anyhow::Result;
use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};

use crate::external::gtfs::agency::Agency;
use crate::external::gtfs::agency_jp::AgencyJp;
use crate::external::gtfs::calendar::Calendar;
use crate::external::gtfs::calendar_dates::CalendarDate;
use crate::external::gtfs::fare_attributes::FareAttribute;
use crate::external::gtfs::fare_rules::FareRule;
use crate::external::gtfs::feed_info::Feed;
use crate::external::gtfs::frequencies::Frequency;
use crate::external::gtfs::office_jp::OfficeJp;
use crate::external::gtfs::routes::Route;
use crate::external::gtfs::routes_jp::RouteJp;
use crate::external::gtfs::shapes::Shape;
use crate::external::gtfs::stop_times::StopTime;
use crate::external::gtfs::stops::Stop;
use crate::external::gtfs::transfers::Transfer;
use crate::external::gtfs::translations::Translation;
use crate::external::gtfs::trips::Trip;

pub mod agency;
pub mod agency_jp;
pub mod calendar;
pub mod calendar_dates;
pub mod fare_attributes;
pub mod fare_rules;
pub mod feed_info;
pub mod frequencies;
pub mod office_jp;
pub mod routes;
pub mod routes_jp;
pub mod shapes;
pub mod stop_times;
pub mod stops;
pub mod transfers;
pub mod translations;
pub mod trips;

/// 色. 00FFFF など 6 桁の 16 進数
pub type Color = String;
/// メートル
pub type Meter = u32;
/// HH:mm:ss形式で28時などの表現も許容
pub type UnlimitedTime = String;
/// YYYY-MM-DD形式の年月日 (ex: 2017-01-06)
pub type DateString = String;
/// YYYYMMDD形式の年月日 (ex: 20170106)(Option<NaiveDate>がDeserializeできないため仕方なく..)
pub type OptionalDateString = Option<String>;
/// 秒
pub type Second = u32;
/// 順序 (ex: 0)
pub type Sequence = u32;
/// 電話番号 (ex: 03-2816-5700)
pub type TelephoneNumber = String;
/// 郵便番号 (ex: 1638001)
pub type ZipNumber = String;
/// メールアドレス
pub type MailAddress = String;
/// 住所 (ex: 東京都新宿区西新宿二丁目８番１号)
pub type Address = String;
/// Url
pub type Url = String;
/// 緯度 (degree)
pub type Latitude = OrderedFloat<f32>;
/// 経度 (degree)
pub type Longitude = OrderedFloat<f32>;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub enum Timezone {
    /// 日本語
    #[serde(rename = "Asia/Tokyo")]
    AsiaTokyo,
}

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
#[serde(rename_all = "lowercase")]
pub enum Lang {
    Ja,
    En,
    Ko,
    #[serde(rename = "zh-CN")]
    ZhCn,
    #[serde(rename = "zh-TW")]
    ZhTw,
    /// ふりがな
    #[serde(rename = "ja-Hrkt")]
    JaHrkt,
}

pub trait Gtfs {
    fn create_all(&self) -> Result<()>;
    fn drop_all(&self) -> Result<()>;
    fn insert_agencies(&mut self, agencies: &[&Agency]) -> Result<()>;
    fn select_agencies(&mut self) -> Result<Vec<Agency>>;
    fn insert_agencies_jp(&mut self, agencies: &[&AgencyJp]) -> Result<()>;
    fn select_agencies_jp(&mut self) -> Result<Vec<AgencyJp>>;
    fn has_agency_jp(&mut self) -> bool;
    fn insert_stops(&mut self, stops: &[&Stop]) -> Result<()>;
    fn select_stops(&mut self) -> Result<Vec<Stop>>;
    fn insert_routes(&mut self, routes: &[&Route]) -> Result<()>;
    fn select_routes(&mut self) -> Result<Vec<Route>>;
    fn insert_routes_jp(&mut self, routes: &[&RouteJp]) -> Result<()>;
    fn select_routes_jp(&mut self) -> Result<Vec<RouteJp>>;
    fn has_routes_jp(&mut self) -> bool;
    fn insert_trips(&mut self, trips: &[&Trip]) -> Result<()>;
    fn select_trips(&mut self) -> Result<Vec<Trip>>;
    fn insert_offices_jp(&mut self, offices: &[&OfficeJp]) -> Result<()>;
    fn select_offices_jp(&mut self) -> Result<Vec<OfficeJp>>;
    fn has_office_jp(&mut self) -> bool;
    fn insert_stop_times(&mut self, stop_times: &[&StopTime]) -> Result<()>;
    fn select_stop_times(&mut self) -> Result<Vec<StopTime>>;
    fn insert_calendars(&mut self, calendars: &[&Calendar]) -> Result<()>;
    fn select_calendars(&mut self) -> Result<Vec<Calendar>>;
    fn insert_calendar_dates(&mut self, calendar_dates: &[&CalendarDate]) -> Result<()>;
    fn select_calendar_dates(&mut self) -> Result<Vec<CalendarDate>>;
    fn has_calendar_dates(&mut self) -> bool;
    fn insert_fare_attributes(&mut self, fare_attributes: &[&FareAttribute]) -> Result<()>;
    fn select_fare_attributes(&mut self) -> Result<Vec<FareAttribute>>;
    fn has_fare_attributes(&mut self) -> bool;
    fn insert_fare_rules(&mut self, fare_rules: &[&FareRule]) -> Result<()>;
    fn select_fare_rules(&mut self) -> Result<Vec<FareRule>>;
    fn has_fare_rules(&mut self) -> bool;
    fn insert_shapes(&mut self, shapes: &[&Shape]) -> Result<()>;
    fn select_shapes(&mut self) -> Result<Vec<Shape>>;
    fn has_shapes(&mut self) -> bool;
    fn insert_frequencies(&mut self, frequencies: &[&Frequency]) -> Result<()>;
    fn select_frequencies(&mut self) -> Result<Vec<Frequency>>;
    fn has_frequencies(&mut self) -> bool;
    fn insert_transfers(&mut self, transfers: &[&Transfer]) -> Result<()>;
    fn select_transfers(&mut self) -> Result<Vec<Transfer>>;
    fn has_transfers(&mut self) -> bool;
    fn insert_feeds(&mut self, feeds: &[&Feed]) -> Result<()>;
    fn select_feeds(&mut self) -> Result<Vec<Feed>>;
    fn insert_translations(&mut self, translations: &[&Translation]) -> Result<()>;
    fn select_translations(&mut self) -> Result<Vec<Translation>>;
}
