use rusqlite::{Connection, NO_PARAMS};
use serde::{Deserialize, Serialize};
use serde_rusqlite::from_rows;

use crate::external::gtfs::extended::service_routes::ServiceRouteId;
use crate::external::gtfs::DirectionId;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct ServiceRouteIdentity {
    /// サービスルートID
    pub service_route_id: ServiceRouteId,
    /// サービスルートの上下区分
    pub service_route_direction_id: DirectionId,
    /// 複数の 便 IDのカンマ区切り (ex: 1001_WD_001, 1001_WD_002)
    pub trip_ids: String,
    /// 複数の 停留所/標柱 IDのカンマ区切り (ex: [停]100,200 [柱]100_10,200_20)
    pub stop_ids: String,
    /// 複数の 停留所/標柱 名称のカンマ区切り (ex: 市役所前,区役所前)
    pub stop_names: String,
}

pub fn select_service_route_identity(
    conn: &mut Connection,
) -> serde_rusqlite::Result<Vec<ServiceRouteIdentity>> {
    let mut stmt = conn.prepare(
        "
SELECT
  service_route_id,
  service_route_direction_id,
  GROUP_CONCAT(trip_id, ',') AS trip_ids,
  stop_names,
  stop_ids
FROM
  (
    SELECT DISTINCT
      trip_id,
      service_route_id,
      service_route_direction_id,
      GROUP_CONCAT(stop_name, ',') AS stop_names,
      GROUP_CONCAT(stop_id, ',')   AS stop_ids
    FROM
      (
        SELECT
          sr.service_route_id AS service_route_id,
          sr.direction_id     AS service_route_direction_id,
          st.stop_name        AS stop_name,
          st.stop_id          AS stop_id,
          stt.trip_id         AS trip_id
        FROM
          stop_times stt
            INNER JOIN trips2service_routes t2sr
            ON stt.trip_id = t2sr.trip_id
            INNER JOIN service_routes sr
            ON t2sr.service_route_id = sr.service_route_id AND t2sr.service_route_direction_id = sr.direction_id
            INNER JOIN stops st
            ON stt.stop_id = st.stop_id
        ORDER BY stt.trip_id, stt.stop_sequence
      )
    GROUP BY
      trip_id
  )
GROUP BY
  service_route_id, service_route_direction_id, stop_ids
ORDER BY
  stop_names, service_route_id, service_route_direction_id;
",
    )?;

    let result = from_rows(stmt.query(NO_PARAMS)?).collect();
    result
}
