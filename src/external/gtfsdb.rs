use std::path::PathBuf;

use anyhow::{Context, Result};
use log::{debug, trace};
use rusqlite::{Connection, NO_PARAMS};
use serde::__private::fmt::Debug;
use serde_rusqlite::{from_rows, to_params_named};

use crate::external::gtfs;
use crate::external::gtfs::agency::Agency;
use crate::external::gtfs::calendar::Calendar;
use crate::external::gtfs::calendar_dates::CalendarDate;
use crate::external::gtfs::fare_attributes::FareAttribute;
use crate::external::gtfs::fare_rules::FareRule;
use crate::external::gtfs::routes::{Route, RouteId};
use crate::external::gtfs::shapes::Shape;
use crate::external::gtfs::stop_times::StopTime;
use crate::external::gtfs::stops::Stop;
use crate::external::gtfs::trips::Trip;
use crate::external::gtfs::Gtfs;

pub struct GtfsDb {
    connection: Connection,
}

pub trait Table {
    fn table_name() -> &'static str;
    fn column_names() -> &'static [&'static str];
    fn create_sql() -> &'static str;
}

pub fn init(path: &PathBuf) -> Result<Box<dyn Gtfs>> {
    let ins = GtfsDb::new(path)?;
    Ok(Box::new(ins))
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

pub fn insert<T>(conn: &mut Connection, records: &[T]) -> rusqlite::Result<()>
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
        tx.execute_named(sql.as_str(), &to_params_named(&record).unwrap().to_slice())?;
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
    pub fn new(db: &PathBuf) -> Result<Self> {
        let db_file = db.to_str().unwrap();
        let conn = Connection::open(db_file)?;

        Ok(GtfsDb { connection: conn })
    }
}

impl Gtfs for GtfsDb {
    fn create_all(&self) -> Result<()> {
        create::<Agency>(&self.connection)?;
        create::<Route>(&self.connection)?;
        create::<Stop>(&self.connection)?;
        create::<Trip>(&self.connection)?;
        create::<StopTime>(&self.connection)?;
        create::<Calendar>(&self.connection)?;
        create::<CalendarDate>(&self.connection)?;
        create::<FareAttribute>(&self.connection)?;
        create::<FareRule>(&self.connection)?;
        create::<Shape>(&self.connection)?;
        Ok(())
    }

    fn drop_all(&self) -> Result<()> {
        drop::<Agency>(&self.connection)?;
        drop::<Route>(&self.connection)?;
        drop::<Stop>(&self.connection)?;
        drop::<Trip>(&self.connection)?;
        drop::<StopTime>(&self.connection)?;
        drop::<Calendar>(&self.connection)?;
        drop::<CalendarDate>(&self.connection)?;
        drop::<FareAttribute>(&self.connection)?;
        drop::<FareRule>(&self.connection)?;
        drop::<Shape>(&self.connection)?;
        Ok(())
    }

    fn insert_agencies(&mut self, agencies: &[Agency]) -> Result<()> {
        insert(&mut self.connection, agencies)?;
        Ok(())
    }

    fn select_agencies(&mut self) -> Result<Vec<Agency>> {
        select_all::<Agency>(&mut self.connection).context("Fail to select agency")
    }

    fn insert_stops(&mut self, stops: &[Stop]) -> Result<()> {
        insert(&mut self.connection, stops)?;
        Ok(())
    }

    fn select_stops(&mut self) -> Result<Vec<Stop>> {
        select_all::<Stop>(&mut self.connection).context("Fail to select stops")
    }

    fn insert_routes(&mut self, routes: &[Route]) -> Result<()> {
        insert(&mut self.connection, routes)?;
        Ok(())
    }

    fn select_routes(&mut self, route_id: &Option<RouteId>) -> Result<Vec<Route>> {
        match route_id {
            Some(id) => gtfs::routes::select_by_route_id(&mut self.connection, id),
            None => select_all::<Route>(&mut self.connection),
        }
        .context("Fail to select_routes")
    }

    fn insert_trips(&mut self, trips: &[Trip]) -> Result<()> {
        insert(&mut self.connection, trips)?;
        Ok(())
    }

    fn select_trips(&mut self, route_id: &Option<RouteId>) -> Result<Vec<Trip>> {
        match route_id {
            Some(id) => gtfs::trips::select_by_route_id(&mut self.connection, id),
            None => select_all::<Trip>(&mut self.connection),
        }
        .context("Fail to select_trips")
    }

    fn insert_stop_times(&mut self, stop_times: &[StopTime]) -> Result<()> {
        insert(&mut self.connection, stop_times)?;
        Ok(())
    }

    fn insert_calendars(&mut self, calendars: &[Calendar]) -> Result<()> {
        insert(&mut self.connection, calendars)?;
        Ok(())
    }

    fn select_calendars(&mut self) -> Result<Vec<Calendar>> {
        select_all::<Calendar>(&mut self.connection).context("Fail to select calendars")
    }

    fn insert_calendar_dates(&mut self, calendar_dates: &[CalendarDate]) -> Result<()> {
        insert(&mut self.connection, calendar_dates)?;
        Ok(())
    }

    fn select_calendar_dates(&mut self) -> Result<Vec<CalendarDate>> {
        select_all::<CalendarDate>(&mut self.connection).context("Fail to select calendar_dates")
    }

    fn insert_fare_attributes(&mut self, fare_attributes: &[FareAttribute]) -> Result<()> {
        insert(&mut self.connection, fare_attributes)?;
        Ok(())
    }

    fn select_fare_attributes(&mut self) -> Result<Vec<FareAttribute>> {
        select_all::<FareAttribute>(&mut self.connection).context("Fail to select fare_attributes")
    }

    fn insert_fare_rules(&mut self, fare_rules: &[FareRule]) -> Result<()> {
        insert(&mut self.connection, fare_rules)?;
        Ok(())
    }

    fn select_fare_rules(&mut self) -> Result<Vec<FareRule>> {
        select_all::<FareRule>(&mut self.connection).context("Fail to select fare_rules")
    }

    fn insert_shapes(&mut self, shapes: &[Shape]) -> Result<()> {
        insert(&mut self.connection, shapes)?;
        Ok(())
    }

    fn select_shapes(&mut self) -> Result<Vec<Shape>> {
        select_all::<Shape>(&mut self.connection).context("Fail to select shapes")
    }
}
