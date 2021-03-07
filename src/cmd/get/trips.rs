use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;

use crate::external::gtfs::Gtfs;
use crate::io::IOType;
use crate::{external, io};

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(short, long, parse(from_os_str), default_value = "hibou.db")]
    database: PathBuf,
    #[clap(short, long)]
    route_id: String,
    /// One of (csv, tsv, json, yaml).
    #[clap(short, long, default_value = "csv")]
    output_type: IOType,
}

pub fn run(op: &Opts) -> Result<()> {
    let mut gtfs = external::gtfsdb::init(&op.database)?;
    let trips = gtfs.select_trips_by_route_id(&op.route_id)?;
    io::write(&trips, &op.output_type)?;
    Ok(())
}
