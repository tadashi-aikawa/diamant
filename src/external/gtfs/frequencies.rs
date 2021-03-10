use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::external::gtfs::trips::TripId;
use crate::external::gtfs::{Second, UnlimitedTime};
use crate::external::gtfsdb::Table;

/// 利用タイプ
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum GuideExactTimes {
    /// 時刻を案内しない
    Yes = 0,
    /// 時刻を案内する
    No = 1,
}

/// 運行間隔情報
/// https://www.gtfs.jp/developpers-guide/format-reference.html#frequencies
#[derive(Debug, Deserialize, Serialize)]
pub struct Frequency {
    /// 便ID
    trip_id: TripId,
    /// 開始時刻
    start_time: UnlimitedTime,
    /// 終了時刻
    end_time: UnlimitedTime,
    /// 運行間隔
    headway_secs: Second,
    /// 案内精度
    exact_times: Option<GuideExactTimes>,
}

impl Table for Frequency {
    fn table_name() -> &'static str {
        "frequencies"
    }

    fn column_names() -> &'static [&'static str] {
        &[
            "trip_id",
            "start_time",
            "end_time",
            "headway_secs",
            "exact_times",
        ]
    }

    fn create_sql() -> &'static str {
        "
        trip_id text not null,
        start_time text not null,
        end_time text not null,
        headway_secs int not null,
        exact_times int,
        PRIMARY KEY(trip_id, start_time)
        "
    }
}
