use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;

use crate::external;
use crate::external::gtfs::Gtfs;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(short, long, parse(from_os_str), default_value = "hibou.db")]
    database: PathBuf,
}

pub fn run(op: &Opts) -> Result<()> {
    let mut gtfs = external::gtfs::create(&op.database)?;
    let trips = gtfs.select_trips()?;
    external::gtfs::write(&trips)?;
    Ok(())
}
