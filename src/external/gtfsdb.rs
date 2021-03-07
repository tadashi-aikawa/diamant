use std::path::PathBuf;

use anyhow::{Context, Result};
use rusqlite::{Connection, NO_PARAMS};

use crate::external::gtfs;
use crate::external::gtfs::agency::Agency;
use crate::external::gtfs::routes::{Route, RouteId};
use crate::external::gtfs::stop_times::StopTime;
use crate::external::gtfs::trips::Trip;
use crate::external::gtfs::Gtfs;
use serde_rusqlite::from_rows;

pub struct GtfsDb {
    connection: Connection,
}

pub fn init(path: &PathBuf) -> Result<Box<dyn Gtfs>> {
    let ins = GtfsDb::new(path)?;
    Ok(Box::new(ins))
}

fn select_all<T>(conn: &mut Connection, table_name: String) -> serde_rusqlite::Result<Vec<T>>
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
        gtfs::routes::create(&self.connection)?;
        gtfs::trips::create(&self.connection)?;
        gtfs::stop_times::create(&self.connection)?;
        Ok(())
    }

    fn drop_all(&self) -> Result<()> {
        gtfs::agency::drop(&self.connection)?;
        gtfs::routes::drop(&self.connection)?;
        gtfs::trips::drop(&self.connection)?;
        gtfs::stop_times::drop(&self.connection)?;
        Ok(())
    }

    fn insert_agencies(&mut self, agencies: &[Agency]) -> Result<()> {
        gtfs::agency::insert(&mut self.connection, &agencies)?;
        Ok(())
    }

    fn select_agencies(&mut self) -> Result<Vec<Agency>> {
        select_all::<Agency>(&mut self.connection, "agency".to_string())
            .context("Fail to select agency")
    }

    fn insert_routes(&mut self, routes: &[Route]) -> Result<()> {
        gtfs::routes::insert(&mut self.connection, &routes)?;
        Ok(())
    }

    fn select_routes(&mut self, route_id: &Option<RouteId>) -> Result<Vec<Route>> {
        match route_id {
            Some(id) => gtfs::routes::select_by_route_id(&mut self.connection, id),
            None => gtfs::routes::select_all(&mut self.connection),
        }
        .context("Fail to select_routes")
    }

    fn insert_stop_times(&mut self, stop_times: &[StopTime]) -> Result<()> {
        gtfs::stop_times::insert(&mut self.connection, &stop_times)?;
        Ok(())
    }

    fn insert_trips(&mut self, trips: &[Trip]) -> Result<()> {
        gtfs::trips::insert(&mut self.connection, &trips)?;
        Ok(())
    }

    fn select_trips(&mut self, route_id: &Option<RouteId>) -> Result<Vec<Trip>> {
        match route_id {
            Some(id) => gtfs::trips::select_by_route_id(&mut self.connection, id),
            None => gtfs::trips::select_all(&mut self.connection),
        }
        .context("Fail to select_trips")
    }
}
