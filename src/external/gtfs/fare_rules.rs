use serde::{Deserialize, Serialize};

use crate::external::gtfs::fare_attributes::FareId;
use crate::external::gtfs::routes::RouteId;
use crate::external::gtfs::stops::ZoneId;
use crate::external::gtfscsv::GTFSFile;
use crate::external::gtfsdb::Table;

/// 運賃定義情報
/// https://www.gtfs.jp/developpers-guide/format-reference.html#fare
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct FareRule {
    /// 運賃ID
    fare_id: FareId,
    /// 経路ID
    route_id: Option<RouteId>,
    /// 乗車地ゾーン
    origin_id: Option<ZoneId>,
    /// 降車地ゾーン
    destination_id: Option<ZoneId>,
    /// 通過ゾーン (JPでは使わない)
    contains_id: Option<ZoneId>,
}

impl GTFSFile for FareRule {
    fn file_name() -> &'static str {
        "fare_rules.txt"
    }
}

impl Table for FareRule {
    fn table_name() -> &'static str {
        "fare_rules"
    }

    fn column_names() -> &'static [&'static str] {
        &[
            "fare_id",
            "route_id",
            "origin_id",
            "destination_id",
            "contains_id",
        ]
    }

    fn create_sql() -> &'static str {
        "
        fare_id text not null,
        route_id text,
        origin_id text,
        destination_id text,
        contains_id text,
        PRIMARY KEY(fare_id, route_id, origin_id, destination_id)
        "
    }
}
