use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::external::gtfs::stops::StopId;
use crate::external::gtfs::trips::TripId;
use crate::external::gtfs::{Meter, Sequence};
use crate::external::gtfsdb::Table;

#[derive(Debug, Deserialize_repr, Serialize_repr, Eq, PartialEq, Clone, Hash)]
#[repr(u8)]
enum PickupType {
    /// 通常の乗車地
    Usual = 0,
    /// 乗車不可能
    Deny = 1,
    /// 交通機関に乗車予約の電話が必要
    NeedOfficeReservation = 2,
    /// 運転手への事前連絡が必要
    NeedDriverReservation = 3,
}

#[derive(Debug, Deserialize_repr, Serialize_repr, Eq, PartialEq, Clone, Hash)]
#[repr(u8)]
enum DropOffType {
    /// 通常の降車地 (ブザーを押して申告する一般的な停留所を含む)
    Usual = 0,
    /// 降車不可能
    Deny = 1,
    /// 交通機関に降車予約の電話が必要
    NeedOfficeReservation = 2,
    /// 乗車時に運転手への事前連絡が必要
    NeedDriverReservation = 3,
}

/// 通過時刻情報
/// https://www.gtfs.jp/developpers-guide/format-reference.html#stop_times
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct StopTime {
    /// 便ID
    trip_id: TripId,
    /// 到着時刻 (ex: 7:00:00)
    arrival_time: String,
    /// 出発時刻 (ex: 7:00:00)
    departure_time: String,
    /// 標柱ID (location_type=0のstopのみ結合可) (ex: 100_10)
    stop_id: StopId,
    /// 通過順位 (ex: 0)
    stop_sequence: Sequence,
    /// 停留所行先 (ex: 東京ビッグサイト（月島駅経由）)
    stop_headsign: Option<String>,
    /// 乗車区分 (ex: 0)
    pickup_type: Option<PickupType>,
    /// 降車区分 (ex: 0)
    drop_off_type: Option<DropOffType>,
    /// 通算距離 (メートル) (ex: 0)
    shape_dist_traveled: Option<Meter>,
    /// 発着時間精度 (日本では使用しない)
    timepoint: Option<i32>,
}

impl Table for StopTime {
    fn table_name() -> &'static str {
        "stop_times"
    }

    fn column_names() -> &'static [&'static str] {
        &[
            "trip_id",
            "arrival_time",
            "departure_time",
            "stop_id",
            "stop_sequence",
            "stop_headsign",
            "pickup_type",
            "drop_off_type",
            "shape_dist_traveled",
            "timepoint",
        ]
    }

    fn create_sql() -> &'static str {
        "
        trip_id text,
        arrival_time datetime not null,
        departure_time datetime not null,
        stop_id text not null,
        stop_sequence int,
        stop_headsign text,
        pickup_type int,
        drop_off_type int,
        shape_dist_traveled int,
        timepoint int,
        PRIMARY KEY(trip_id, stop_sequence),
        foreign key (trip_id) references trips(trip_id),
        foreign key (stop_id) references stops(stop_id)
        "
    }
}
