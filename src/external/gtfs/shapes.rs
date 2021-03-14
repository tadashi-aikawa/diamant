use serde::{Deserialize, Serialize};

use crate::external::gtfs::{Latitude, Longitude, Sequence};
use crate::external::gtfsdb::Table;
use ordered_float::OrderedFloat;

/// 描画ID (ex: S_1001)
pub type ShapeId = String;

/// 描画情報
/// https://www.gtfs.jp/developpers-guide/format-reference.html#shapes
#[derive(Debug, Deserialize, Serialize, PartialEq, Clone, Eq, Hash)]
pub struct Shape {
    /// 描画ID
    shape_id: ShapeId,
    /// 描画緯度
    shape_pt_lat: Latitude,
    /// 描画経度
    shape_pt_lon: Longitude,
    /// 描画順序
    shape_pt_sequence: Sequence,
    /// 描画距離 (JPでは使わない)
    shape_dist_traveled: Option<OrderedFloat<f32>>,
}

impl Table for Shape {
    fn table_name() -> &'static str {
        "shapes"
    }

    fn column_names() -> &'static [&'static str] {
        &[
            "shape_id",
            "shape_pt_lat",
            "shape_pt_lon",
            "shape_pt_sequence",
            "shape_dist_traveled",
        ]
    }

    fn create_sql() -> &'static str {
        "
        shape_id text not null,
        shape_pt_lat double not null,
        shape_pt_lon double not null,
        shape_pt_sequence int not null,
        shape_dist_traveled double,
        PRIMARY KEY(shape_id, shape_pt_sequence)
        "
    }
}
