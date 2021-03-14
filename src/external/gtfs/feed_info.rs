use serde::{Deserialize, Serialize};

use crate::external::gtfs::{DateString, Lang, Url};
use crate::external::gtfsdb::Table;

/// 提供情報
/// https://www.gtfs.jp/developpers-guide/format-reference.html#feed_info
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct Feed {
    /// 提供組織名 (ex: 東京都交通局)
    feed_publisher_name: String,
    /// 提供組織 URL
    feed_publisher_url: Url,
    /// 提供言語
    feed_lang: Lang,
    /// 有効期間開始日
    feed_start_date: Option<DateString>,
    /// 有効期間終了日
    feed_end_date: Option<DateString>,
    /// 提供データバージョン
    feed_version: Option<String>,
}

impl Table for Feed {
    fn table_name() -> &'static str {
        "feed_info"
    }

    fn column_names() -> &'static [&'static str] {
        &[
            "feed_publisher_name",
            "feed_publisher_url",
            "feed_lang",
            "feed_start_date",
            "feed_end_date",
            "feed_version",
        ]
    }

    fn create_sql() -> &'static str {
        "
        feed_publisher_name text primary key,
        feed_publisher_url text not null,
        feed_lang int not null,
        feed_start_date text,
        feed_end_date text,
        feed_version text
        "
    }
}
