use rusqlite::{Connection, NO_PARAMS};
use serde::{Deserialize, Serialize};

use crate::external::gtfs::stop_times::StopTime;
use crate::external::gtfs::stops::{Stop, StopId};
use crate::external::gtfs::trips::{Trip, TripId};
use crate::external::gtfs::{Latitude, Longitude, Sequence};

use crate::external::gtfs::routes::{Route, RouteId};
use crate::external::gtfsdb::Table;
use serde_rusqlite::from_rows;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct TripWithSequenceMeta {
    /// 便ID
    pub trip_id: TripId,
    /// 通過順位 (ex: 0)
    pub stop_sequence: Sequence,
    /// 標柱ID
    pub stop_id: StopId,
    /// 停留所・標柱名称 (ex: ①東京駅八重洲口 ②東京駅八重洲口)
    pub stop_name: String,
    /// 緯度 (ex: ①35.680515 ※ターミナル中心 ②35.679752 ※標柱位置)
    pub stop_lat: Latitude,
    /// 経度 (ex: ①139.764698 ※ターミナル中心）②139.768330 ※標柱位置)
    pub stop_lon: Longitude,
    /// 経路ID
    pub route_id: RouteId,
    /// 経路略称 (ex: 東16)
    pub route_short_name: Option<String>,
    /// 経路名 (ex: 東京駅八重洲口～月島駅前～東京ビ ッグサイト)
    pub route_long_name: Option<String>,
}

impl TripWithSequenceMeta {
    pub fn route_name(self) -> String {
        match self.route_long_name {
            Some(n) => n,
            None => self.route_short_name.unwrap(), // long_nameとshort_nameどちらかは値がある
        }
    }
}

pub fn select_trip_with_sequence_meta(
    conn: &mut Connection,
) -> serde_rusqlite::Result<Vec<TripWithSequenceMeta>> {
    let mut stmt = conn.prepare(
        format!(
            "
SELECT
  stt.trip_id,
  stt.stop_sequence,
  st.stop_id,
  st.stop_name,
  st.stop_lat,
  st.stop_lon,
  r.route_id,
  r.route_short_name,
  r.route_long_name
FROM
  {} stt
    INNER JOIN {} t
    ON stt.trip_id == t.trip_id
    INNER JOIN {} st
    ON stt.stop_id == st.stop_id
    INNER JOIN {} r
    ON t.route_id == r.route_id
ORDER BY
  stt.trip_id, stt.stop_sequence
",
            StopTime::table_name(),
            Trip::table_name(),
            Stop::table_name(),
            Route::table_name(),
        )
        .as_str(),
    )?;
    let result = from_rows(stmt.query(NO_PARAMS)?).collect();
    result
}
