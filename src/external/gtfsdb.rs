use std::path::Path;

use anyhow::{Context, Result};
use log::{debug, trace};
use rusqlite::{Connection, NO_PARAMS};
use serde::__private::fmt::Debug;
use serde_rusqlite::{from_rows, to_params_named};

use crate::external::gtfs::agency::Agency;
use crate::external::gtfs::agency_jp::AgencyJp;
use crate::external::gtfs::calendar::Calendar;
use crate::external::gtfs::calendar_dates::CalendarDate;
use crate::external::gtfs::extended::course::Course;
use crate::external::gtfs::extended::stop_time_details::{
    select_stop_time_details, select_stop_time_details_by_ids, select_stop_time_details_by_name,
    StopTimeDetail,
};
use crate::external::gtfs::extended::trips2courses::Trip2Course;
use crate::external::gtfs::fare_attributes::FareAttribute;
use crate::external::gtfs::fare_rules::FareRule;
use crate::external::gtfs::feed_info::Feed;
use crate::external::gtfs::frequencies::Frequency;
use crate::external::gtfs::legacy_translations::LegacyTranslation;
use crate::external::gtfs::office_jp::OfficeJp;
use crate::external::gtfs::routes::Route;
use crate::external::gtfs::routes_jp::RouteJp;
use crate::external::gtfs::shapes::Shape;
use crate::external::gtfs::stop_times::StopTime;
use crate::external::gtfs::stops::{select_stops_by_name, Stop, StopId};
use crate::external::gtfs::transfers::Transfer;
use crate::external::gtfs::translations::Translation;
use crate::external::gtfs::trips::{select_trips_by_stop, Trip, TripId};
use crate::external::gtfs::GtfsDbTrait;

pub struct GtfsDb {
    connection: Connection,
}

pub trait Table {
    fn table_name() -> &'static str;
    fn column_names() -> &'static [&'static str];
    fn create_sql() -> &'static str;
}

pub fn init(path: &Path) -> Result<GtfsDb> {
    GtfsDb::new(path)
}

pub fn create<T>(conn: &Connection) -> Result<()>
where
    T: Table,
{
    conn.execute(
        format!(
            "CREATE TABLE IF NOT EXISTS {} ({})",
            T::table_name(),
            T::create_sql()
        )
        .as_str(),
        NO_PARAMS,
    )?;
    debug!("Create table `{}`", T::table_name());
    Ok(())
}

pub fn drop<T>(conn: &Connection) -> Result<()>
where
    T: Table,
{
    conn.execute(
        format!("DROP TABLE IF EXISTS {}", T::table_name()).as_str(),
        NO_PARAMS,
    )?;
    debug!("Drop table `{}`", T::table_name());
    Ok(())
}

pub fn insert<T>(conn: &mut Connection, records: &[T]) -> anyhow::Result<()>
where
    T: serde::ser::Serialize + Debug + Table,
{
    let tx = conn.transaction()?;

    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        T::table_name(),
        T::column_names().join(","),
        T::column_names()
            .iter()
            .map(|x| format!(":{}", x))
            .collect::<Vec<_>>()
            .join(","),
    );

    debug!("Insert {} records to {}", records.len(), T::table_name());
    for record in records {
        tx.execute_named(sql.as_str(), &to_params_named(&record).unwrap().to_slice())
            .with_context(|| format!("Fail to insert {:?}", record))?;
    }

    tx.commit()?;

    trace!("Insert {:?}", records);
    Ok(())
}

fn select_all<T>(conn: &mut Connection) -> serde_rusqlite::Result<Vec<T>>
where
    T: serde::de::DeserializeOwned + Table,
{
    let mut stmt = conn.prepare(format!("SELECT * FROM {}", T::table_name()).as_str())?;
    let result = from_rows::<T>(stmt.query(NO_PARAMS)?).collect();
    result
}

impl GtfsDb {
    pub fn new(db: &Path) -> Result<Self> {
        let conn = Connection::open(db)?;
        rusqlite::vtab::array::load_module(&conn)?;

        Ok(GtfsDb { connection: conn })
    }
}

impl GtfsDbTrait for GtfsDb {
    fn create_all(&self) -> Result<()> {
        // TODO: GTFSとGTFS-JPでmethodを分けた方がいいかも、他に独自テーブルの有無もあるし
        create::<Agency>(&self.connection)?;
        create::<AgencyJp>(&self.connection)?;
        create::<Stop>(&self.connection)?;
        create::<Route>(&self.connection)?;
        create::<RouteJp>(&self.connection)?;
        create::<Trip>(&self.connection)?;
        create::<OfficeJp>(&self.connection)?;
        create::<StopTime>(&self.connection)?;
        create::<Calendar>(&self.connection)?;
        create::<CalendarDate>(&self.connection)?;
        create::<FareAttribute>(&self.connection)?;
        create::<FareRule>(&self.connection)?;
        create::<Shape>(&self.connection)?;
        create::<Frequency>(&self.connection)?;
        create::<Transfer>(&self.connection)?;
        create::<Feed>(&self.connection)?;
        create::<Translation>(&self.connection)?;
        // ----------- extended ---------------
        create::<Trip2Course>(&self.connection)?;
        create::<Course>(&self.connection)?;
        Ok(())
    }

    fn drop_all(&self) -> Result<()> {
        // TODO: GTFSとGTFS-JPでmethodを分けた方がいいかも、他に独自テーブルの有無もあるし
        drop::<Agency>(&self.connection)?;
        drop::<AgencyJp>(&self.connection)?;
        drop::<Stop>(&self.connection)?;
        drop::<Route>(&self.connection)?;
        drop::<RouteJp>(&self.connection)?;
        drop::<Trip>(&self.connection)?;
        drop::<OfficeJp>(&self.connection)?;
        drop::<StopTime>(&self.connection)?;
        drop::<Calendar>(&self.connection)?;
        drop::<CalendarDate>(&self.connection)?;
        drop::<FareAttribute>(&self.connection)?;
        drop::<FareRule>(&self.connection)?;
        drop::<Shape>(&self.connection)?;
        drop::<Frequency>(&self.connection)?;
        drop::<Transfer>(&self.connection)?;
        drop::<Feed>(&self.connection)?;
        drop::<Translation>(&self.connection)?;
        // ----------- extended ---------------
        drop::<Trip2Course>(&self.connection)?;
        drop::<Course>(&self.connection)?;
        Ok(())
    }

    fn insert_agencies(&mut self, agencies: &[Agency]) -> Result<()> {
        insert(&mut self.connection, agencies)
    }

    fn insert_agencies_jp(&mut self, agencies_jp: &[AgencyJp]) -> Result<()> {
        insert(&mut self.connection, agencies_jp)
    }

    fn insert_stops(&mut self, stops: &[Stop]) -> Result<()> {
        insert(&mut self.connection, stops)
    }

    fn select_stops(&mut self, word: String) -> Result<Vec<Stop>> {
        select_stops_by_name(&mut self.connection, word).context("Fail to select_stops_by_name")
    }

    fn insert_routes(&mut self, routes: &[Route]) -> Result<()> {
        insert(&mut self.connection, routes)
    }

    fn select_routes(&mut self) -> Result<Vec<Route>> {
        select_all::<Route>(&mut self.connection).context("Fail to select_routes")
    }

    fn insert_routes_jp(&mut self, routes_jp: &[RouteJp]) -> Result<()> {
        insert(&mut self.connection, routes_jp)
    }

    fn insert_trips(&mut self, trips: &[Trip]) -> Result<()> {
        insert(&mut self.connection, trips)
    }

    fn select_trips(&mut self, stop_id: StopId) -> Result<Vec<Trip>> {
        select_trips_by_stop(&mut self.connection, stop_id).context("Fail to select_trips_by_stop")
    }

    fn insert_offices_jp(&mut self, offices: &[OfficeJp]) -> Result<()> {
        insert(&mut self.connection, offices)
    }

    fn insert_stop_times(&mut self, stop_times: &[StopTime]) -> Result<()> {
        insert(&mut self.connection, stop_times)
    }

    fn insert_calendars(&mut self, calendars: &[Calendar]) -> Result<()> {
        insert(&mut self.connection, calendars)
    }

    fn insert_calendar_dates(&mut self, calendar_dates: &[CalendarDate]) -> Result<()> {
        insert(&mut self.connection, calendar_dates)
    }

    fn insert_fare_attributes(&mut self, fare_attributes: &[FareAttribute]) -> Result<()> {
        insert(&mut self.connection, fare_attributes)
    }

    fn insert_fare_rules(&mut self, fare_rules: &[FareRule]) -> Result<()> {
        insert(&mut self.connection, fare_rules)
    }

    fn insert_shapes(&mut self, shapes: &[Shape]) -> Result<()> {
        insert(&mut self.connection, shapes)
    }

    fn insert_frequencies(&mut self, frequencies: &[Frequency]) -> Result<()> {
        insert(&mut self.connection, frequencies)
    }

    fn insert_transfers(&mut self, transfers: &[Transfer]) -> Result<()> {
        insert(&mut self.connection, transfers)
    }

    fn insert_feeds(&mut self, feeds: &[Feed]) -> Result<()> {
        insert(&mut self.connection, feeds)
    }

    fn insert_translations(&mut self, translations: &[Translation]) -> Result<()> {
        insert(&mut self.connection, translations)
    }

    fn insert_legacy_translations(&mut self, translations: &[LegacyTranslation]) -> Result<()> {
        insert(&mut self.connection, translations)
    }

    /// ---------------------------extended--------------------------

    fn select_stop_time_details(
        &mut self,
        trip_ids: Option<Vec<TripId>>,
        stop_name_prefix: Option<String>,
    ) -> Result<Vec<StopTimeDetail>> {
        match (trip_ids, stop_name_prefix) {
            (Some(ids), _) => select_stop_time_details_by_ids(&mut self.connection, ids),
            (_, Some(stop_name_prefix)) => {
                select_stop_time_details_by_name(&mut self.connection, stop_name_prefix)
            }
            _ => select_stop_time_details(&mut self.connection),
        }
        .context("Fail to select_trip_with_stops")
    }

    fn insert_trips2courses(&mut self, trip2courses: &[Trip2Course]) -> Result<()> {
        insert(&mut self.connection, trip2courses)
    }

    fn insert_courses(&mut self, courses: &[Course]) -> Result<()> {
        insert(&mut self.connection, courses)
    }
}
