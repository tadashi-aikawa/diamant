use std::path::PathBuf;

use crate::external::gtfs::routes::Route;
use crate::external::gtfs::stop_times::StopTime;
use crate::external::gtfs::trips::Trip;
use crate::{external, io};
use anyhow::Result;
use clap::Clap;
use log::info;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(parse(from_os_str))]
    gtfs_dir: PathBuf,
    #[clap(short, long, parse(from_os_str), default_value = "hibou.db")]
    database: PathBuf,
}

pub fn run(op: &Opts) -> Result<()> {
    let mut gtfs = external::gtfsdb::init(&op.database)?;

    info!("ℹ️ Initialize.");
    gtfs.drop_all()?;
    gtfs.create_all()?;
    info!("  ✨ Success");

    let routes = io::read::<Route>(&op.gtfs_dir.join("routes.txt"))?;
    info!("ℹ️ Add {} routes.", routes.len());
    gtfs.insert_routes(&routes)?;
    info!("  ✨ Success");

    let trips = io::read::<Trip>(&op.gtfs_dir.join("trips.txt"))?;
    info!("ℹ️ Add {} trips.", trips.len());
    gtfs.insert_trips(&trips)?;
    info!("  ✨ Success");

    let stop_times = io::read::<StopTime>(&op.gtfs_dir.join("stop_times.txt"))?;
    info!("ℹ️ Add {} stop_times.", stop_times.len());
    gtfs.insert_stop_times(&stop_times)?;
    info!("  ✨ Success");

    Ok(())
}
