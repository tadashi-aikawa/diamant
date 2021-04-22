use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;
use strum::VariantNames;

use crate::app::trip::TripServiceDb;
use crate::io::Format;
use crate::{external, io};

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(short, long, parse(from_os_str), default_value = "gtfs.db")]
    database: PathBuf,
    #[clap(short, long, default_value = "csv", possible_values(Format::VARIANTS))]
    format: Format,
}

pub fn run(op: &Opts) -> Result<()> {
    let gtfs = external::gtfsdb::GtfsDb::new(&op.database)?;
    let metas = TripServiceDb::new(gtfs).fetch_trip_with_sequence_metas(None)?;
    io::write(&metas, &op.format)?;
    Ok(())
}
