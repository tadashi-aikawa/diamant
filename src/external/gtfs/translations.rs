use serde::{Deserialize, Serialize};

use crate::external::gtfs::legacy_translations::LegacyTranslation;
use crate::external::gtfs::Lang;
use crate::external::gtfscsv::GTFSFile;
use crate::external::gtfsdb::Table;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
#[serde(rename_all = "snake_case")]
pub enum TranslatableTableName {
    Agency,
    Stops,
    Routes,
    Trips,
    StopTimes,
    FeedInfo,
    Pathways,
    Levels,
    Attributions,
}

/// 翻訳情報
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct Translation {
    /// テーブル名
    table_name: TranslatableTableName,
    /// フィールド名
    field_name: String,
    /// 言語
    language: Lang,
    /// 翻訳済み値
    translation: String,
    /// レコードID
    record_id: Option<String>,
    /// レコードサブID
    record_sub_id: Option<String>,
    /// フィールド値
    field_value: Option<String>,
}

impl GTFSFile for Translation {
    fn file_name() -> &'static str {
        "translations.txt"
    }
}

impl Table for Translation {
    fn table_name() -> &'static str {
        "translations"
    }

    fn column_names() -> &'static [&'static str] {
        &[
            "table_name",
            "field_name",
            "language",
            "translation",
            "record_id",
            "record_sub_id",
            "field_value",
        ]
    }

    fn create_sql() -> &'static str {
        "
        table_name text not null,
        field_name text not null,
        language text not null,
        translation text not null,
        record_id text,
        record_sub_id text,
        field_value text
        "
    }
}

impl Translation {
    /// 出現頻度の高いもののみ
    pub fn from_legacy(regacy: &LegacyTranslation) -> Vec<Translation> {
        vec![
            Translation {
                table_name: TranslatableTableName::Stops,
                field_name: "stop_name".to_string(),
                language: regacy.lang.clone(),
                translation: regacy.translation.clone(),
                record_id: None,
                record_sub_id: None,
                field_value: Some(regacy.trans_id.clone()),
            },
            Translation {
                table_name: TranslatableTableName::Routes,
                field_name: "route_short_name".to_string(),
                language: regacy.lang.clone(),
                translation: regacy.translation.clone(),
                record_id: None,
                record_sub_id: None,
                field_value: Some(regacy.trans_id.clone()),
            },
            Translation {
                table_name: TranslatableTableName::Routes,
                field_name: "route_long_name".to_string(),
                language: regacy.lang.clone(),
                translation: regacy.translation.clone(),
                record_id: None,
                record_sub_id: None,
                field_value: Some(regacy.trans_id.clone()),
            },
            Translation {
                table_name: TranslatableTableName::Trips,
                field_name: "trip_headsign".to_string(),
                language: regacy.lang.clone(),
                translation: regacy.translation.clone(),
                record_id: None,
                record_sub_id: None,
                field_value: Some(regacy.trans_id.clone()),
            },
            Translation {
                table_name: TranslatableTableName::Trips,
                field_name: "trip_short_name".to_string(),
                language: regacy.lang.clone(),
                translation: regacy.translation.clone(),
                record_id: None,
                record_sub_id: None,
                field_value: Some(regacy.trans_id.clone()),
            },
            Translation {
                table_name: TranslatableTableName::StopTimes,
                field_name: "stop_headsign".to_string(),
                language: regacy.lang.clone(),
                translation: regacy.translation.clone(),
                record_id: None,
                record_sub_id: None,
                field_value: Some(regacy.trans_id.clone()),
            },
        ]
    }
}
