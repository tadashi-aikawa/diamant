use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;
use log::info;

use crate::external::gtfs::agency::Agency;
use crate::external::gtfs::calendar::Calendar;
use crate::external::gtfs::routes::Route;
use crate::external::gtfs::stops::Stop;
use crate::external::gtfs::trips::Trip;
use crate::{external, io};

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
    info!("  ℹ️ Drop all tables.");
    gtfs.drop_all()?;
    info!("  ℹ️ Create all tables.");
    gtfs.create_all()?;
    info!("  ✨ Success");

    let agencies = io::read::<Agency>(&op.gtfs_dir.join("agency.txt"))?;
    info!("ℹ️ [agencies] {} records", agencies.len());
    gtfs.insert_agencies(&agencies)?;
    info!("  ✨ Success");

    let stops = io::read::<Stop>(&op.gtfs_dir.join("stops.txt"))?;
    info!("ℹ️ [stops] {} records", stops.len());
    gtfs.insert_stops(&stops)?;
    info!("  ✨ Success");

    let routes = io::read::<Route>(&op.gtfs_dir.join("routes.txt"))?;
    info!("ℹ️ [routes] {} records", routes.len());
    gtfs.insert_routes(&routes)?;
    info!("  ✨ Success");

    let trips = io::read::<Trip>(&op.gtfs_dir.join("trips.txt"))?;
    info!("ℹ️ [trips] {} records", trips.len());
    gtfs.insert_trips(&trips)?;
    info!("  ✨ Success");

    // let stop_times = io::read::<StopTime>(&op.gtfs_dir.join("stop_times.txt"))?;
    // info!("ℹ️ [stop_times] {} records", stop_times.len());
    // gtfs.insert_stop_times(&stop_times)?;
    // info!("  ✨ Success");

    let calendars = io::read::<Calendar>(&op.gtfs_dir.join("calendar.txt"))?;
    info!("ℹ️ [calendar] {} records", calendars.len());
    gtfs.insert_calendars(&calendars)?;
    info!("  ✨ Success");

    Ok(())
}
