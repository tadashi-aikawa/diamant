use rusqlite::{Connection, NO_PARAMS};
use serde::{Deserialize, Serialize};
use serde_rusqlite::from_rows;

use crate::external::gtfs::stops::StopId;
use crate::external::gtfs::{Latitude, Longitude};

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct StopDetail {
    /// 停留所・標柱ID
    pub stop_id: StopId,
    /// 停留所・標柱名称 (ex: ①東京駅八重洲口 ②東京駅八重洲口)
    pub stop_name: String,
    /// 読み仮名
    pub stop_ruby: String,
    /// 緯度 (ex: ①35.680515 ※ターミナル中心 ②35.679752 ※標柱位置)
    pub stop_lat: Latitude,
    /// 経度 (ex: ①139.764698 ※ターミナル中心）②139.768330 ※標柱位置)
    pub stop_lon: Longitude,
    /// 親駅情報
    /// location_typeが
    ///   - 0だと任意
    ///   - 1だと利用不可
    ///   - 2～4だと必須
    pub parent_station: Option<StopId>,
}

pub fn select_stop_details(conn: &mut Connection) -> serde_rusqlite::Result<Vec<StopDetail>> {
    let mut stmt = conn.prepare(
        "
SELECT
  st.stop_id,
  st.stop_name,
  t.translation as stop_ruby,
  st.stop_lat,
  st.stop_lon,
  st.parent_station
FROM
  stops st
    LEFT JOIN translations t
    ON t.table_name == 'stops' AND t.field_name == 'stop_name' AND t.language == 'ja-Hrkt' AND
       st.stop_name == t.field_value;
",
    )?;

    let result = from_rows(stmt.query(NO_PARAMS)?).collect();
    result
}
