use crate::external::gtfsdb::Table;
use crate::serde_chrono_custom::yyyymmdd;
use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 運行日ID (ex: 平日(月～金))
pub type ServiceId = String;

/// 運行状態
#[derive(Debug, Deserialize_repr, Serialize_repr, Eq, PartialEq, Clone, Hash)]
#[repr(u8)]
enum OperationStatus {
    /// 非運行
    Absent = 0,
    /// 運行
    Present = 1,
}

/// 運行区分情報
/// https://www.gtfs.jp/developpers-guide/format-reference.html#calendar
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct Calendar {
    /// 運行日ID
    service_id: ServiceId,
    /// 月曜日
    monday: OperationStatus,
    /// 火曜日
    tuesday: OperationStatus,
    /// 水曜日
    wednesday: OperationStatus,
    /// 木曜日
    thursday: OperationStatus,
    /// 金曜日
    friday: OperationStatus,
    /// 土曜日
    saturday: OperationStatus,
    /// 日曜日
    sunday: OperationStatus,
    /// サービス開始日
    #[serde(with = "yyyymmdd")]
    start_date: NaiveDate,
    /// サービス終了日
    #[serde(with = "yyyymmdd")]
    end_date: NaiveDate,
}

impl Table for Calendar {
    fn table_name() -> &'static str {
        "calendar"
    }

    fn column_names() -> &'static [&'static str] {
        &[
            "service_id",
            "monday",
            "tuesday",
            "wednesday",
            "thursday",
            "friday",
            "saturday",
            "sunday",
            "start_date",
            "end_date",
        ]
    }

    fn create_sql() -> &'static str {
        "
        service_id text primary key,
        monday int not null,
        tuesday int not null,
        wednesday int not null,
        thursday int not null,
        friday int not null,
        saturday int not null,
        sunday int not null,
        start_date text not null,
        end_date text not null
        "
    }
}
