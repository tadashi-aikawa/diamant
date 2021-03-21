use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

use crate::external::gtfs::stops::StopId;
use crate::external::gtfs::Second;
use crate::external::gtfscsv::GTFSFile;
use crate::external::gtfsdb::Table;

/// 利用タイプ
#[derive(Debug, Deserialize_repr, Serialize_repr, Eq, PartialEq, Clone, Hash)]
#[repr(u8)]
enum TransferType {
    /// 2つの経路間の推奨乗換地点
    Recommended = 0,
    /// 2つの経路間で時間に余裕のある乗換地点
    Afford = 1,
    /// 2つの経路間で時間ギリギリの乗換地点
    Barely = 2,
    /// 2つの経路間で乗換が不可能
    Impossible = 3,
}

/// 乗換情報
/// https://www.gtfs.jp/developpers-guide/format-reference.html#transfers
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct Transfer {
    /// 乗換元標柱ID
    from_stop_id: StopId,
    /// 乗換先標柱ID
    to_stop_id: StopId,
    /// 乗換タイプ
    transfer_type: TransferType,
    /// 乗換時間
    min_transfer_time: Option<Second>,
}

impl GTFSFile for Transfer {
    fn file_name() -> &'static str {
        "transfers.txt"
    }
}

impl Table for Transfer {
    fn table_name() -> &'static str {
        "transfers"
    }

    fn column_names() -> &'static [&'static str] {
        &[
            "from_stop_id",
            "to_stop_id",
            "transfer_type",
            "min_transfer_time",
        ]
    }

    fn create_sql() -> &'static str {
        "
        from_stop_id text not null,
        to_stop_id text not null,
        transfer_type int not null,
        min_transfer_time int,
        PRIMARY KEY(from_stop_id, to_stop_id)
        "
    }
}
