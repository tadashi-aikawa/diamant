use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;

use crate::app::gtfs::GtfsService;
use crate::external;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(parse(from_os_str))]
    gtfs_dir: PathBuf,
    #[clap(short, long, parse(from_os_str), default_value = "hibou.db")]
    database: PathBuf,
    /// Load legacy translations and create a current translations table
    #[clap(short, long)]
    legacy_translations: bool,
}

pub fn run(op: &Opts) -> Result<()> {
    let gtfs_csv = external::gtfscsv::GtfsCsv::new(&op.gtfs_dir)?;
    let gtfs_db = external::gtfsdb::init(&op.database)?;

    let mut service = GtfsService::new(gtfs_csv, gtfs_db);

    service.drop_tables()?;
    service.create_tables()?;
    service.insert_tables(op.legacy_translations)?;

    Ok(())
}
