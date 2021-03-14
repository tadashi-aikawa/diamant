use crate::external::gtfs::Second;
use crate::external::gtfsdb::Table;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};

/// 運賃ID (ex: F_210)
pub type FareId = String;
/// 通過 (ex: JPY)
pub type CurrencyType = String;

/// 利用タイプ
#[derive(Debug, Deserialize_repr, Serialize_repr, Eq, PartialEq, Clone, Hash)]
#[repr(u8)]
enum PaymentMethod {
    /// 乗車後に支払う
    AfterRide = 0,
    /// 乗車前に支払う
    BeforeRide = 1,
}

/// 乗換回数
#[derive(Debug, Deserialize_repr, Serialize_repr, Eq, PartialEq, Clone, Hash)]
#[repr(u8)]
enum TransferCount {
    /// 乗り換え不可
    None = 0,
    /// 1度の乗り換えが可能
    Once = 1,
    /// 2度の乗り換えが可能
    Twice = 2,
}

/// 運賃属性情報
/// https://www.gtfs.jp/developpers-guide/format-reference.html#fare
#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct FareAttribute {
    /// 運賃ID
    fare_id: FareId,
    /// 運賃
    price: i32,
    /// 通過
    currency_type: CurrencyType,
    /// 支払いタイミング
    payment_method: PaymentMethod,
    /// 乗換 (未指定の場合は乗り換え回数の制限なし)
    transfers: Option<TransferCount>,
    /// 乗換有効期限
    transfer_duration: Option<Second>,
}

impl Table for FareAttribute {
    fn table_name() -> &'static str {
        "fare_attributes"
    }

    fn column_names() -> &'static [&'static str] {
        &[
            "fare_id",
            "price",
            "currency_type",
            "payment_method",
            "transfers",
            "transfer_duration",
        ]
    }

    fn create_sql() -> &'static str {
        "
        fare_id text,
        price int not null,
        currency_type text,
        payment_method int not null,
        transfers int,
        transfer_duration int,
        PRIMARY KEY(fare_id, currency_type)
        "
    }
}
