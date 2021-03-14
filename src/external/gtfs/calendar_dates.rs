use crate::external::gtfsdb::Table;
use crate::serde_chrono_custom::yyyymmdd;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 運行日ID (ex: 平日(月～金))
pub type ServiceId = String;

/// 利用タイプ
#[derive(Debug, Deserialize_repr, Serialize_repr, Eq, PartialEq, Clone, Hash)]
#[repr(u8)]
enum ExceptionType {
    /// 運行区分適用
    Apply = 1,
    /// 運行区分非適用
    NotApply = 2,
}

/// 運行区分情報
/// https://www.gtfs.jp/developpers-guide/format-reference.html#calendar
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct CalendarDate {
    /// サービスID
    service_id: ServiceId,
    /// 日付
    #[serde(with = "yyyymmdd")]
    date: NaiveDate,
    /// 利用タイプ
    exception_type: ExceptionType,
}

impl Table for CalendarDate {
    fn table_name() -> &'static str {
        "calendar_dates"
    }

    fn column_names() -> &'static [&'static str] {
        &["service_id", "date", "exception_type"]
    }

    fn create_sql() -> &'static str {
        "
        service_id text,
        date text,
        exception_type int not null,
        PRIMARY KEY(service_id, date)
        "
    }
}
