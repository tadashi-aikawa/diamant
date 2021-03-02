use std::path::PathBuf;

use anyhow::Result;
use rusqlite::Connection;

use crate::external::gtfs;
use crate::external::gtfs::routes::Route;
use crate::external::gtfs::stop_times::StopTime;
use crate::external::gtfs::trips::Trip;
use crate::external::gtfs::Gtfs;

pub struct GtfsDb {
    connection: Connection,
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
        gtfs::routes::create(&self.connection)?;
        gtfs::trips::create(&self.connection)?;
        gtfs::stop_times::create(&self.connection)?;
        Ok(())
    }

    fn drop_all(&self) -> Result<()> {
        gtfs::routes::drop(&self.connection)?;
        gtfs::trips::drop(&self.connection)?;
        gtfs::stop_times::drop(&self.connection)?;
        Ok(())
    }

    fn insert_routes(&mut self, routes: &[Route]) -> Result<()> {
        gtfs::routes::insert(&mut self.connection, &routes)?;
        Ok(())
    }

    fn insert_stop_times(&mut self, stop_times: &[StopTime]) -> Result<()> {
        gtfs::stop_times::insert(&mut self.connection, &stop_times)?;
        Ok(())
    }

    fn insert_trips(&mut self, trips: &[Trip]) -> Result<()> {
        gtfs::trips::insert(&mut self.connection, &trips)?;
        Ok(())
    }

    fn select_trips(&mut self) -> Result<Vec<Trip>> {
        let h = gtfs::trips::select(&mut self.connection)?;
        Ok(h)
    }
}
