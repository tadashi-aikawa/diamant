use log::{debug, trace};
use rusqlite::NO_PARAMS;
use rusqlite::{params, Connection};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Trip {
    /// 経路ID (ex: 1001)
    route_id: String,
    /// 運行ID (ex: 平日(月～金))
    service_id: String,
    /// 便ID (ex: 1001_WD_001)
    trip_id: String,
    /// 便行き先 (ex: 東京ビッグサイト（月島駅経由）)
    trip_headsign: Option<String>,
    /// 便名称
    trip_short_name: Option<String>,
    /// 上下区分 (0：復路　1：往路)
    direction_id: Option<i32>,
    /// 便結合区分
    block_id: Option<String>,
    /// 描画ID (ex: S_1001)
    shape_id: Option<String>,
    /// 車いす利用区分 (0: 車いすによる乗車可否の情報なし　1: 少なくとも1台の車いすによる乗車可能　2: 車いすによる乗車不可)
    wheelchair_accessible: Option<i32>,
    /// 自転車持込区分 (0: 自転車の持込可否の情報なし　1: 少なくとも1台の自転車の持込可能　2: 自転車の持込不可)
    bikes_allowed: Option<i32>,
    /// 便情報
    jp_trip_desc: Option<String>,
    /// 便記号
    jp_trip_desc_symbol: Option<String>,
    /// 営業所ID (ex: S)
    jp_office_id: Option<String>,
}

pub fn create(conn: &Connection) -> rusqlite::Result<()> {
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

pub fn drop(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("DROP TABLE IF EXISTS trips", NO_PARAMS)?;
    debug!("Drop table `trips`");
    Ok(())
}

pub fn insert(conn: &mut Connection, trips: &[Trip]) -> rusqlite::Result<()> {
    let tx = conn.transaction()?;

    debug!("Insert {} records to trips", trips.len());
    for trip in trips {
        trace!("Insert {:?}", trip);
        tx.execute(
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
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11, ?12, ?13)",
            params![
                trip.route_id,
                trip.service_id,
                trip.trip_id,
                trip.trip_headsign,
                trip.trip_short_name,
                trip.direction_id,
                trip.block_id,
                trip.shape_id,
                trip.wheelchair_accessible,
                trip.bikes_allowed,
                trip.jp_trip_desc,
                trip.jp_trip_desc_symbol,
                trip.jp_office_id,
            ],
        )?;
    }

    tx.commit()?;

    Ok(())
}
