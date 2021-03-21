use serde::{Deserialize, Serialize};

use crate::external::gtfs::agency::AgencyId;
use crate::external::gtfs::{Address, ZipNumber};
use crate::external::gtfsdb::Table;

/// 事業者情報(JP)
/// https://www.gtfs.jp/developpers-guide/format-reference.html#agency
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct AgencyJp {
    /// 事業者ID
    agency_id: AgencyId,
    /// 事業者正式名称 (ex: 東京都交通局)
    agency_official_name: Option<String>,
    /// 事業者郵便番号
    agency_zip_number: Option<ZipNumber>,
    /// 事業者住所
    agency_address: Option<Address>,
    /// 代表者肩書 (ex: 局長)
    agency_president_pos: Option<String>,
    /// 代表者氏名 (ex: 東京 太郎)
    agency_president_name: Option<String>,
}

impl Table for AgencyJp {
    fn table_name() -> &'static str {
        "agency_jp"
    }

    fn column_names() -> &'static [&'static str] {
        &[
            "agency_id",
            "agency_official_name",
            "agency_zip_number",
            "agency_address",
            "agency_president_pos",
            "agency_president_name",
        ]
    }

    fn create_sql() -> &'static str {
        "
        agency_id text primary key,
        agency_official_name text,
        agency_zip_number text,
        agency_address text,
        agency_president_pos text,
        agency_president_name text
        "
    }
}
