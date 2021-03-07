use log::debug;
use rusqlite::named_params;
use rusqlite::Connection;
use rusqlite::NO_PARAMS;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_rusqlite::from_rows;

use crate::external::gtfs::agency::AgencyId;
use crate::external::gtfs::Color;
use crate::external::gtfsdb::Table;

/// 経路ID (ex: 1001)
pub type RouteId = String;

// GTFSの場合は他にも追加されるはず
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum RouteType {
    /// バス
    BUS = 3,
}

/// 経路情報
/// https://www.gtfs.jp/developpers-guide/format-reference.html#routes
#[derive(Debug, Deserialize, Serialize)]
pub struct Route {
    /// 経路ID
    route_id: RouteId,
    /// 事業者ID
    agency_id: AgencyId,
    /// 経路略称 (ex: 東16)
    /// route_long_nameとどちらか1つは指定必須
    route_short_name: Option<String>,
    /// 経路名 (ex: 東京駅八重洲口～月島駅前～東京ビ ッグサイト)
    /// route_long_nameとどちらか1つは指定必須
    route_long_name: Option<String>,
    /// 経路情報
    route_desc: Option<String>,
    /// 経路タイプ
    route_type: RouteType,
    /// 経路URL (ex: http://tobus.jp/blsys/navi?LCD=&VCD=cslrsi &ECD=picsroute&RTM CD=50)
    route_url: Option<String>,
    /// 経路色 (ex: FFD700)
    route_color: Option<Color>,
    /// 経路文字色 (ex: 000000)
    route_text_color: Option<Color>,
    /// 路線ID
    jp_parent_route_id: Option<String>,
}

impl Table for Route {
    fn table_name() -> &'static str {
        "routes"
    }

    fn column_names() -> &'static [&'static str] {
        &[
            "route_id",
            "agency_id",
            "route_short_name",
            "route_long_name",
            "route_desc",
            "route_type",
            "route_url",
            "route_color",
            "route_text_color",
            "jp_parent_route_id",
        ]
    }
}

pub fn create(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS routes (
            route_id text primary key,
            agency_id text not null,
            route_short_name text,
            route_long_name text,
            route_desc text,
            route_type int not null,
            route_url text,
            route_color text,
            route_text_color text,
            jp_parent_route_id text
        )",
        NO_PARAMS,
    )?;
    debug!("Create table `routes`");
    Ok(())
}

pub fn select_by_route_id(
    conn: &mut Connection,
    route_id: &RouteId,
) -> serde_rusqlite::Result<Vec<Route>> {
    let mut stmt = conn.prepare("SELECT * FROM routes WHERE route_id = :route_id")?;
    let result =
        from_rows::<Route>(stmt.query_named(named_params! {":route_id": route_id})?).collect();
    result
}
