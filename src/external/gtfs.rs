use std::io;
use std::path::PathBuf;
use strum_macros::EnumString;

use anyhow::{Context, Result};
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::external::gtfs::routes::Route;
use crate::external::gtfs::stop_times::StopTime;
use crate::external::gtfs::trips::{RouteId, Trip};
use crate::external::gtfsdb::GtfsDb;

pub mod routes;
pub mod stop_times;
pub mod trips;

pub trait Gtfs {
    fn create_all(&self) -> Result<()>;
    fn drop_all(&self) -> Result<()>;
    fn insert_routes(&mut self, routes: &[Route]) -> Result<()>;
    fn insert_stop_times(&mut self, stop_times: &[StopTime]) -> Result<()>;
    fn insert_trips(&mut self, trips: &[Trip]) -> Result<()>;
    fn select_trips_by_route_id(&mut self, route_id: &RouteId) -> Result<Vec<Trip>>;
}

pub fn init(path: &PathBuf) -> Result<impl Gtfs> {
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

#[derive(Debug, EnumString)]
#[strum(serialize_all = "snake_case")]
pub enum WriterType {
    CSV,
    TSV,
    JSON,
    YAML,
}

pub fn write<T>(records: &[T], output_type: &WriterType) -> Result<()>
where
    T: Serialize,
{
    match output_type {
        WriterType::CSV => write_csv(records, b','),
        WriterType::TSV => write_csv(records, b'\t'),
        WriterType::JSON => write_json(records),
        WriterType::YAML => write_yaml(records),
    }?;
    Ok(())
}

fn write_csv<T>(records: &[T], delimiter: u8) -> Result<()>
where
    T: Serialize,
{
    let mut wtr = csv::WriterBuilder::new()
        .delimiter(delimiter)
        .from_writer(io::stdout());
    for r in records {
        wtr.serialize(r)?;
        wtr.flush()?;
    }
    Ok(())
}

fn write_json<T>(records: &[T]) -> Result<()>
where
    T: Serialize,
{
    serde_json::to_writer(io::stdout(), records)?;
    Ok(())
}

fn write_yaml<T>(records: &[T]) -> Result<()>
where
    T: Serialize,
{
    serde_yaml::to_writer(io::stdout(), records)?;
    Ok(())
}
