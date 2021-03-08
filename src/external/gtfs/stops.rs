use crate::external::gtfs::{Timezone, Url, ZoneId};
use crate::external::gtfsdb::Table;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 停留所・標柱ID (ex: ①100 ②100_10)
pub type StopId = String;
/// のりばコード (『番』『のりば』のような語句は含めない. 翻訳言語に応じて変わるため)
pub type PlatformCode = String;

/// 停留所・標柱区分
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum LocationType {
    /// 標柱
    Pole = 0,
    /// 停留所
    Stop = 1,
}

/// 停留所・標柱情報
/// https://www.gtfs.jp/developpers-guide/format-reference.html#stops
#[derive(Debug, Deserialize, Serialize)]
pub struct Stop {
    /// 停留所・標柱ID
    stop_id: StopId,
    /// 停留所・標柱番号
    stop_code: Option<String>,
    /// 停留所・標柱名称 (ex: ①東京駅八重洲口 ②東京駅八重洲口)
    stop_name: String,
    /// 停留所・標柱付加情報
    stop_desc: Option<String>,
    /// 緯度 (ex: ①35.680515 ※ターミナル中心 ②35.679752 ※標柱位置)
    stop_lat: f32,
    /// 経度 (ex: ①139.764698 ※ターミナル中心）②139.768330 ※標柱位置)
    stop_lon: f32,
    /// 運賃エリアID (ex: ①設定しない ②Z_210　※都区内エリアID)
    zone_id: Option<ZoneId>,
    /// 停留所・標柱URL
    stop_url: Option<Url>,
    /// 停留所・標柱区分
    location_type: Option<LocationType>,
    /// 親駅情報
    parent_station: Option<StopId>,
    /// タイムゾーン (日本ではagency_timezoneが優先されるため不要)
    stop_timezone: Option<Timezone>,
    /// 車椅子情報 (日本のバスでは設定しなそうなのでenum定義しない)
    wheelchair_boarding: Option<u32>,
    /// のりば情報 (ex: ①※設定なし ②10)
    platform_code: Option<PlatformCode>,
}

impl Table for Stop {
    fn table_name() -> &'static str {
        "stops"
    }

    fn column_names() -> &'static [&'static str] {
        &[
            "stop_id",
            "stop_code",
            "stop_name",
            "stop_desc",
            "stop_lat",
            "stop_lon",
            "zone_id",
            "stop_url",
            "location_type",
            "parent_station",
            "stop_timezone",
            "wheelchair_boarding",
            "platform_code",
        ]
    }

    fn create_sql() -> &'static str {
        "
        stop_id text primary key,
        stop_code text,
        stop_name text not null,
        stop_desc text,
        stop_lat double,
        stop_lon double,
        zone_id text,
        stop_url text,
        location_type int,
        parent_station text,
        stop_timezone text,
        wheelchair_boarding int,
        platform_code text
        "
    }
}
