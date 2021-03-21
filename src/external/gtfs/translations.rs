use serde::{Deserialize, Serialize};

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
