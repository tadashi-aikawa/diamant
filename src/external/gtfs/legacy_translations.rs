use serde::{Deserialize, Serialize};

use crate::external::gtfs::Lang;
use crate::external::gtfscsv::GTFSFile;
use crate::external::gtfsdb::Table;

/// 古いGTFSの翻訳情報
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct LegacyTranslation {
    /// 翻訳元日本語 (ex: 数寄屋橋)
    pub trans_id: String,
    /// 言語
    pub lang: Lang,
    /// 翻訳済み値
    pub translation: String,
}

impl GTFSFile for LegacyTranslation {
    fn file_name() -> &'static str {
        "translations.txt"
    }
}

impl Table for LegacyTranslation {
    fn table_name() -> &'static str {
        "translations"
    }

    fn column_names() -> &'static [&'static str] {
        &["table_id", "lang", "translation"]
    }

    fn create_sql() -> &'static str {
        "
        trans_id text not null,
        lang text not null,
        translation text not null,
        PRIMARY KEY(trans_id, lang)
        "
    }
}
