use crate::external::gtfs::routes::RouteId;
use anyhow::Result;
use log::debug;
use rusqlite::named_params;
use rusqlite::Connection;
use rusqlite::NO_PARAMS;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_rusqlite::from_rows;

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum Direction {
    /// 往路
    Outbound = 0,
    /// 復路
    Inbound = 1,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum WheelchairAccessible {
    /// 車いすによる乗車可否の情報なし
    Unknown = 0,
    /// 少なくとも1台の車いすによる乗車可能
    Allow = 1,
    /// 車いすによる乗車不可
    Deny = 2,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum BikesAllowed {
    /// 自転車の持込可否の情報なし
    Unknown = 0,
    /// 少なくとも1台の自転車の持込可能
    Allow = 1,
    /// 自転車の持込不可
    Deny = 2,
}

/// 便ID (ex: 1001_WD_001)
pub type TripId = String;

/// 運行日ID (ex: 平日(月～金))
type ServiceId = String;
/// 営業所ID (ex: S)
type JpOfficeId = String;

pub const TABLE_NAME: &str = "trips";
pub const COLUMNS: &[&str] = &[
    "route_id",
    "service_id",
    "trip_id",
    "trip_headsign",
    "trip_short_name",
    "direction_id",
    "block_id",
    "shape_id",
    "wheelchair_accessible",
    "bikes_allowed",
    "jp_trip_desc",
    "jp_trip_desc_symbol",
    "jp_office_id",
];

/// 便情報
/// https://www.gtfs.jp/developpers-guide/format-reference.html#trips
#[derive(Debug, Deserialize, Serialize)]
pub struct Trip {
    /// 経路ID
    route_id: RouteId,
    /// 運行日ID
    service_id: ServiceId,
    /// 便ID
    trip_id: TripId,
    /// 便行き先 (ex: 東京ビッグサイト（月島駅経由）)
    trip_headsign: Option<String>,
    /// 便名称
    trip_short_name: Option<String>,
    /// 上下区分
    direction_id: Option<Direction>,
    /// 便結合区分
    block_id: Option<String>,
    /// 描画ID (ex: S_1001)
    shape_id: Option<String>,
    /// 車いす利用区分
    wheelchair_accessible: Option<WheelchairAccessible>,
    /// 自転車持込区分
    bikes_allowed: Option<BikesAllowed>,
    /// 便情報
    jp_trip_desc: Option<String>,
    /// 便記号
    jp_trip_desc_symbol: Option<String>,
    /// 営業所ID
    jp_office_id: Option<JpOfficeId>,
}

pub fn create(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS trips (
            route_id text not null,
            service_id text not null,
            trip_id text primary key,
            trip_headsign text,
            trip_short_name text,
            direction_id int,
            block_id text,
            shape_id text,
            wheelchair_accessible int,
            bikes_allowed int,
            jp_trip_desc text,
            jp_trip_desc_symbol text,
            jp_office_id text
        )",
        NO_PARAMS,
    )?;
    debug!("Create table `trips`");
    Ok(())
}

pub fn select_by_route_id(
    conn: &mut Connection,
    route_id: &RouteId,
) -> serde_rusqlite::Result<Vec<Trip>> {
    let mut stmt = conn.prepare("SELECT * FROM trips WHERE route_id = :route_id")?;
    let result =
        from_rows::<Trip>(stmt.query_named(named_params! {":route_id": route_id})?).collect();
    result
}
