use crate::external::gtfs::{Lang, MailAddress, TelephoneNumber, Timezone, Url};
use log::{debug, trace};
use rusqlite::Connection;
use rusqlite::NO_PARAMS;
use serde::{Deserialize, Serialize};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_rusqlite::to_params_named;

/// 事業者ID  (ex: 8000020130001, 8000020130001_1)
pub type AgencyId = String;

// GTFSの場合は他にも追加されるはず
#[derive(Debug, Deserialize_repr, Serialize_repr)]
#[repr(u8)]
enum RouteType {
    /// バス
    BUS = 3,
}

/// 経路情報
/// https://www.gtfs.jp/developpers-guide/format-reference.html#agency
#[derive(Debug, Deserialize, Serialize)]
pub struct Agency {
    /// 事業者ID
    agency_id: AgencyId,
    /// 事業者名称 (ex: 都営バス)
    agency_name: String,
    /// 事業者URL (ex: http://www.kotsu.metro.tokyo.jp/bus/)
    agency_url: Url,
    /// タイムゾーン (ex: Asia/Tokyo)
    agency_timezone: Timezone,
    /// 言語
    agency_lang: Lang,
    /// 電話番号
    agency_phone: Option<TelephoneNumber>,
    /// オンライン購入URL
    agency_fare_url: Option<Url>,
    /// 事業者Eメール
    agency_email: Option<MailAddress>,
}

pub fn create(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS agency (
            agency_id text primary key,
            agency_name text not null,
            agency_url text not null,
            agency_timezone text not null,
            agency_lang text not null,
            agency_phone text,
            agency_fare_url text,
            agency_email text
        )",
        NO_PARAMS,
    )?;
    debug!("Create table `agency`");
    Ok(())
}

pub fn drop(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("DROP TABLE IF EXISTS agency", NO_PARAMS)?;
    debug!("Drop table `agency`");
    Ok(())
}

pub fn insert(conn: &mut Connection, agencies: &[Agency]) -> rusqlite::Result<()> {
    let tx = conn.transaction()?;

    debug!("Insert {} records to agency", agencies.len());
    for agency in agencies {
        trace!("Insert {:?}", agency);
        tx.execute_named(
            "INSERT INTO agency (
            agency_id,
            agency_name,
            agency_url,
            agency_timezone,
            agency_lang,
            agency_phone,
            agency_fare_url,
            agency_email
        ) VALUES (
            :agency_id,
            :agency_name,
            :agency_url,
            :agency_timezone,
            :agency_lang,
            :agency_phone,
            :agency_fare_url,
            :agency_email
        )",
            &to_params_named(&agency).unwrap().to_slice(),
        )?;
    }

    tx.commit()?;

    Ok(())
}
