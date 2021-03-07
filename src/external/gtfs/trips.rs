use anyhow::Result;
use log::{debug, trace};
use rusqlite::named_params;
use rusqlite::Connection;
use rusqlite::NO_PARAMS;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_rusqlite::{from_rows, to_params_named};
use serde_with::skip_serializing_none;

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum Direction {
    /// 往路
    OUTBOUND = 0,
    /// 復路
    INBOUND = 1,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum WheelchairAccessible {
    /// 車いすによる乗車可否の情報なし
    UNKNOWN = 0,
    /// 少なくとも1台の車いすによる乗車可能
    ALLOW = 1,
    /// 車いすによる乗車不可
    DENY = 2,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum BikesAllowed {
    /// 自転車の持込可否の情報なし
    UNKNOWN = 0,
    /// 少なくとも1台の自転車の持込可能
    ALLOW = 1,
    /// 自転車の持込不可
    DENY = 2,
}

/// 便ID (ex: 1001_WD_001)
type TripId = String;

/// 経路ID (ex: 1001)
pub type RouteId = String;
/// 運行ID (ex: 平日(月～金))
type ServiceId = String;
/// 営業所ID (ex: S)
type JpOfficeId = String;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Trip {
    route_id: RouteId,
    service_id: ServiceId,
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

pub fn insert(conn: &mut Connection, trips: &[Trip]) -> Result<()> {
    let tx = conn.transaction()?;

    debug!("Insert {} records to trips", trips.len());
    for trip in trips {
        trace!("Insert {:?}", trip);
        tx.execute_named(
            "INSERT INTO trips (
            route_id,
            service_id,
            trip_id,
            trip_headsign,
            trip_short_name,
            direction_id,
            block_id,
            shape_id,
            wheelchair_accessible,
            bikes_allowed,
            jp_trip_desc,
            jp_trip_desc_symbol,
            jp_office_id
        ) VALUES (
            :route_id,
            :service_id,
            :trip_id,
            :trip_headsign,
            :trip_short_name,
            :direction_id,
            :block_id,
            :shape_id,
            :wheelchair_accessible,
            :bikes_allowed,
            :jp_trip_desc,
            :jp_trip_desc_symbol,
            :jp_office_id
        )",
            &to_params_named(&trip).unwrap().to_slice(),
        )?;
    }

    tx.commit()?;

    Ok(())
}

pub fn drop(conn: &Connection) -> Result<()> {
    conn.execute("DROP TABLE IF EXISTS trips", NO_PARAMS)?;
    debug!("Drop table `trips`");
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
