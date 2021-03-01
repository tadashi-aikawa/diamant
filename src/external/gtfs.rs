use crate::external::gtfs::routes::Route;
use crate::external::gtfs::stop_times::StopTime;
use crate::external::gtfs::trips::Trip;
use crate::external::gtfsdb::GtfsDb;
use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use std::path::PathBuf;

pub mod routes;
pub mod stop_times;
pub mod trips;

pub trait Gtfs {
    fn create_all(&self) -> Result<()>;
    fn drop_all(&self) -> Result<()>;
    fn insert_routes(&mut self, routes: &[Route]) -> Result<()>;
    fn insert_stop_times(&mut self, stop_times: &[StopTime]) -> Result<()>;
    fn insert_trips(&mut self, trips: &[Trip]) -> Result<()>;
}

pub fn create(path: &PathBuf) -> Result<impl Gtfs> {
    let ins = GtfsDb::new(path)?;
    Ok(ins)
}

pub fn read<T>(path: &PathBuf) -> Result<Vec<T>>
where
    T: DeserializeOwned,
{
    let r: Result<Vec<_>, _> = csv::Reader::from_path(path)
        .with_context(|| format!("Could not read file {:?}", &path.to_str()))?
        .deserialize()
        .collect();
    r.with_context(|| "エラー")
}
