use crate::external::gtfs::trips::TripId;
use crate::external::gtfs::{Meter, Sequence, StopId};
use log::{debug, trace};
use rusqlite::Connection;
use rusqlite::NO_PARAMS;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_rusqlite::to_params_named;

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum PickupType {
    /// 通常の乗車地
    Usual = 0,
    /// 乗車不可能
    Deny = 1,
    /// 交通機関に乗車予約の電話が必要
    NeedOfficeReservation = 2,
    /// 運転手への事前連絡が必要
    NeedDriverReservation = 3,
}

#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum DropOffType {
    /// 通常の降車地 (ブザーを押して申告する一般的な停留所を含む)
    Usual = 0,
    /// 降車不可能
    Deny = 1,
    /// 交通機関に降車予約の電話が必要
    NeedOfficeReservation = 2,
    /// 乗車時に運転手への事前連絡が必要
    NeedDriverReservation = 3,
}

/// 通過時刻情報
/// https://www.gtfs.jp/developpers-guide/format-reference.html#stop_times
#[derive(Debug, Deserialize, Serialize)]
pub struct StopTime {
    /// 便ID
    trip_id: TripId,
    /// 到着時刻 (ex: 7:00:00)
    arrival_time: String,
    /// 出発時刻 (ex: 7:00:00)
    departure_time: String,
    /// 標柱ID (location_type=0のstopのみ結合可) (ex: 100_10)
    stop_id: StopId,
    /// 通過順位 (ex: 0)
    stop_sequence: Sequence,
    /// 停留所行先 (ex: 東京ビッグサイト（月島駅経由）)
    stop_headsign: Option<String>,
    /// 乗車区分 (ex: 0)
    pickup_type: Option<PickupType>,
    /// 降車区分 (ex: 0)
    drop_off_type: Option<DropOffType>,
    /// 通算距離 (メートル) (ex: 0)
    shape_dist_traveled: Option<Meter>,
    /// 発着時間精度 (日本では使用しない)
    timepoint: Option<i32>,
}

pub fn create(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stop_times (
            trip_id text,
            arrival_time datetime not null,
            departure_time datetime not null,
            stop_id text not null,
            stop_sequence int,
            stop_headsign text,
            pickup_type int,
            drop_off_type int,
            shape_dist_traveled int,
            timepoint int,
            PRIMARY KEY(trip_id, stop_sequence)
        )",
        NO_PARAMS,
    )?;
    debug!("Create table `stop_times`");
    Ok(())
}

pub fn drop(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("DROP TABLE IF EXISTS stop_times", NO_PARAMS)?;
    debug!("Drop table `stop_times`");
    Ok(())
}

pub fn insert(conn: &mut Connection, stop_times: &[StopTime]) -> rusqlite::Result<()> {
    let tx = conn.transaction()?;

    debug!("Insert {} records to stop_times", stop_times.len());
    for stop_time in stop_times {
        tx.execute_named(
            "INSERT INTO stop_times (
            trip_id,
            arrival_time,
            departure_time,
            stop_id,
            stop_sequence,
            stop_headsign,
            pickup_type,
            drop_off_type,
            shape_dist_traveled,
            timepoint
        ) VALUES (
            :trip_id,
            :arrival_time,
            :departure_time,
            :stop_id,
            :stop_sequence,
            :stop_headsign,
            :pickup_type,
            :drop_off_type,
            :shape_dist_traveled,
            :timepoint
        )",
            &to_params_named(&stop_time).unwrap().to_slice(),
        )?;
    }

    tx.commit()?;

    trace!("Insert {:?}", stop_times);
    Ok(())
}
