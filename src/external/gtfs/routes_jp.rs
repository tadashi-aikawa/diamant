use serde::{Deserialize, Serialize};

use crate::external::gtfs::routes::RouteId;
use crate::external::gtfs::OptionalDateString;
use crate::external::gtfsdb::Table;

/// 経路情報(JP)
/// https://www.gtfs.jp/developpers-guide/format-reference.html#routes
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct RouteJp {
    /// 経路ID
    route_id: RouteId,
    /// ダイヤ改正日
    route_update_date: OptionalDateString,
    /// 起点 (ex: 東京駅八重洲口)
    origin_stop: Option<String>,
    /// 経過地 (ex: 月島駅)
    via_stop: Option<String>,
    /// 終点 (ex: 東京ビッグサイト)
    destination_stop: Option<String>,
}

impl Table for RouteJp {
    fn table_name() -> &'static str {
        "routes_jp"
    }

    fn column_names() -> &'static [&'static str] {
        &[
            "route_id",
            "route_update_date",
            "origin_stop",
            "via_stop",
            "destination_stop",
        ]
    }

    fn create_sql() -> &'static str {
        "
        route_id text primary key,
        route_update_date text,
        origin_stop text,
        via_stop text,
        destination_stop text,
        foreign key (route_id) references routes(route_id)
        "
    }
}
