use serde::{Deserialize, Serialize};

use crate::external::gtfs::{TelephoneNumber, Url};
use crate::external::gtfscsv::GTFSFile;
use crate::external::gtfsdb::Table;

/// 営業所ID (ex: S)
pub type JpOfficeId = String;

/// 営業所情報(JP)
/// https://www.gtfs.jp/developpers-guide/format-reference.html#office_jp
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct OfficeJp {
    /// 営業所ID
    office_id: JpOfficeId,
    /// 営業所名 (ex: 深川営業所)
    office_name: String,
    /// 営業所URL
    office_url: Option<Url>,
    /// 営業所電話番号
    office_phone: Option<TelephoneNumber>,
}

impl GTFSFile for OfficeJp {
    fn file_name() -> &'static str {
        "office_jp.txt"
    }
}

impl Table for OfficeJp {
    fn table_name() -> &'static str {
        "office_jp"
    }

    fn column_names() -> &'static [&'static str] {
        &["office_id", "office_name", "office_url", "office_phone"]
    }

    fn create_sql() -> &'static str {
        "
        office_id text primary key,
        office_name text not null,
        office_url text,
        office_phone text
        "
    }
}
