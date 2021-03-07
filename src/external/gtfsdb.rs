use log::{debug, trace};
use std::path::PathBuf;

use anyhow::{Context, Result};
use rusqlite::{Connection, NO_PARAMS};

use crate::external::gtfs;
use crate::external::gtfs::agency::Agency;
use crate::external::gtfs::routes::{Route, RouteId};
use crate::external::gtfs::stop_times::StopTime;
use crate::external::gtfs::stops::Stop;
use crate::external::gtfs::trips::Trip;
use crate::external::gtfs::Gtfs;
use serde::__private::fmt::Debug;
use serde_rusqlite::{from_rows, to_params_named};

pub struct GtfsDb {
    connection: Connection,
}

pub fn init(path: &PathBuf) -> Result<Box<dyn Gtfs>> {
    let ins = GtfsDb::new(path)?;
    Ok(Box::new(ins))
}

pub fn insert<T>(
    conn: &mut Connection,
    records: &[T],
    table_name: &str,
    column_names: &[&str],
) -> rusqlite::Result<()>
where
    T: serde::ser::Serialize + Debug,
{
    let tx = conn.transaction()?;

    let sql = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name,
        column_names.join(","),
        column_names
            .iter()
            .map(|x| format!(":{}", x))
            .collect::<Vec<_>>()
            .join(","),
    );

    debug!("Insert {} records to {}", records.len(), table_name);
    for record in records {
        tx.execute_named(sql.as_str(), &to_params_named(&record).unwrap().to_slice())?;
    }

    tx.commit()?;

    trace!("Insert {:?}", records);
    Ok(())
}

pub fn drop(conn: &Connection, table_name: &str) -> Result<()> {
    conn.execute(
        format!("DROP TABLE IF EXISTS {}", table_name).as_str(),
        NO_PARAMS,
    )?;
    debug!("Drop table `{}`", table_name);
    Ok(())
}

fn select_all<T>(conn: &mut Connection, table_name: &str) -> serde_rusqlite::Result<Vec<T>>
where
    T: serde::de::DeserializeOwned,
{
    let mut stmt = conn.prepare(format!("SELECT * FROM {}", table_name).as_str())?;
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
        gtfs::agency::create(&self.connection)?;
        gtfs::stops::create(&self.connection)?;
        gtfs::routes::create(&self.connection)?;
        gtfs::trips::create(&self.connection)?;
        gtfs::stop_times::create(&self.connection)?;
        Ok(())
    }

    fn drop_all(&self) -> Result<()> {
        drop(&self.connection, gtfs::agency::TABLE_NAME)?;
        drop(&self.connection, gtfs::routes::TABLE_NAME)?;
        drop(&self.connection, gtfs::stop_times::TABLE_NAME)?;
        drop(&self.connection, gtfs::stops::TABLE_NAME)?;
        drop(&self.connection, gtfs::trips::TABLE_NAME)?;
        Ok(())
    }

    fn insert_agencies(&mut self, agencies: &[Agency]) -> Result<()> {
        insert(
            &mut self.connection,
            agencies,
            gtfs::agency::TABLE_NAME,
            gtfs::agency::COLUMNS,
        )?;
        Ok(())
    }

    fn select_agencies(&mut self) -> Result<Vec<Agency>> {
        select_all::<Agency>(&mut self.connection, gtfs::agency::TABLE_NAME)
            .context("Fail to select agency")
    }

    fn insert_stops(&mut self, stops: &[Stop]) -> Result<()> {
        insert(
            &mut self.connection,
            stops,
            gtfs::stops::TABLE_NAME,
            gtfs::stops::COLUMNS,
        )?;
        Ok(())
    }

    fn select_stops(&mut self) -> Result<Vec<Stop>> {
        select_all::<Stop>(&mut self.connection, gtfs::stops::TABLE_NAME)
            .context("Fail to select stops")
    }

    fn insert_routes(&mut self, routes: &[Route]) -> Result<()> {
        insert(
            &mut self.connection,
            routes,
            gtfs::routes::TABLE_NAME,
            gtfs::routes::COLUMNS,
        )?;
        Ok(())
    }

    fn select_routes(&mut self, route_id: &Option<RouteId>) -> Result<Vec<Route>> {
        match route_id {
            Some(id) => gtfs::routes::select_by_route_id(&mut self.connection, id),
            None => select_all::<Route>(&mut self.connection, gtfs::routes::TABLE_NAME),
        }
        .context("Fail to select_routes")
    }

    fn insert_stop_times(&mut self, stop_times: &[StopTime]) -> Result<()> {
        insert(
            &mut self.connection,
            stop_times,
            gtfs::stop_times::TABLE_NAME,
            gtfs::stop_times::COLUMNS,
        )?;
        Ok(())
    }

    fn insert_trips(&mut self, trips: &[Trip]) -> Result<()> {
        insert(
            &mut self.connection,
            trips,
            gtfs::trips::TABLE_NAME,
            gtfs::trips::COLUMNS,
        )?;
        Ok(())
    }

    fn select_trips(&mut self, route_id: &Option<RouteId>) -> Result<Vec<Trip>> {
        match route_id {
            Some(id) => gtfs::trips::select_by_route_id(&mut self.connection, id),
            None => select_all::<Trip>(&mut self.connection, gtfs::trips::TABLE_NAME),
        }
        .context("Fail to select_trips")
    }
}
