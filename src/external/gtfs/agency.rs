use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::external::gtfs::{Lang, MailAddress, TelephoneNumber, Timezone, Url};
use crate::external::gtfsdb::Table;

/// 事業者ID  (ex: 8000020130001, 8000020130001_1)
pub type AgencyId = String;

// GTFSの場合は他にも追加されるはず
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum RouteType {
    /// バス
    BUS = 3,
}

/// 経路情報
/// https://www.gtfs.jp/developpers-guide/format-reference.html#agency
#[derive(Debug, Deserialize, Serialize)]
pub struct Agency {
    /// 事業者ID
    agency_id: AgencyId,
    /// 事業者名称 (ex: 都営バス)
    agency_name: String,
    /// 事業者URL (ex: http://www.kotsu.metro.tokyo.jp/bus/)
    agency_url: Url,
    /// タイムゾーン (ex: Asia/Tokyo)
    agency_timezone: Timezone,
    /// 言語
    agency_lang: Lang,
    /// 電話番号
    agency_phone: Option<TelephoneNumber>,
    /// オンライン購入URL
    agency_fare_url: Option<Url>,
    /// 事業者Eメール
    agency_email: Option<MailAddress>,
}

impl Table for Agency {
    fn table_name() -> &'static str {
        "agency"
    }

    fn column_names() -> &'static [&'static str] {
        &[
            "agency_id",
            "agency_name",
            "agency_url",
            "agency_timezone",
            "agency_lang",
            "agency_phone",
            "agency_fare_url",
            "agency_email",
        ]
    }

    fn create_sql() -> &'static str {
        "
        agency_id text primary key,
        agency_name text not null,
        agency_url text not null,
        agency_timezone text not null,
        agency_lang text not null,
        agency_phone text,
        agency_fare_url text,
        agency_email text
        "
    }
}
