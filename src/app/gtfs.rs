use anyhow::Result;
use log::info;

use crate::external::gtfs::agency::Agency;
use crate::external::gtfs::calendar::Calendar;
use crate::external::gtfs::calendar_dates::CalendarDate;
use crate::external::gtfs::fare_attributes::FareAttribute;
use crate::external::gtfs::fare_rules::FareRule;
use crate::external::gtfs::feed_info::Feed;
use crate::external::gtfs::frequencies::Frequency;
use crate::external::gtfs::routes::Route;
use crate::external::gtfs::stops::Stop;
use crate::external::gtfs::transfers::Transfer;
use crate::external::gtfs::trips::Trip;
use crate::{external, io};
use std::path::PathBuf;

pub struct GtfsService {
    gtfs: Box<dyn external::gtfs::Gtfs>,
}

/// GTFS全体を横断するアプリケーションサービス
impl GtfsService {
    pub fn new(gtfs: Box<dyn external::gtfs::Gtfs>) -> Self {
        Self { gtfs }
    }

    pub fn create_tables(&mut self) -> Result<()> {
        info!("ℹ️ Create all tables.");
        self.gtfs.create_all()?;
        info!("  ✨ Success");
        Ok(())
    }

    pub fn insert_tables(&mut self, gtfs_dir: &PathBuf) -> Result<()> {
        let agencies = io::read::<Agency>(&gtfs_dir.join("agency.txt"))?;
        info!("ℹ️ [agencies] {} records", agencies.len());
        self.gtfs.insert_agencies(&agencies)?;
        info!("  ✨ Success");

        let stops = io::read::<Stop>(&gtfs_dir.join("stops.txt"))?;
        info!("ℹ️ [stops] {} records", stops.len());
        self.gtfs.insert_stops(&stops)?;
        info!("  ✨ Success");

        let routes = io::read::<Route>(&gtfs_dir.join("routes.txt"))?;
        info!("ℹ️ [routes] {} records", routes.len());
        self.gtfs.insert_routes(&routes)?;
        info!("  ✨ Success");

        let trips = io::read::<Trip>(&gtfs_dir.join("trips.txt"))?;
        info!("ℹ️ [trips] {} records", trips.len());
        self.gtfs.insert_trips(&trips)?;
        info!("  ✨ Success");

        // let stop_times = io::read::<StopTime>(&gtfs_dir.join("stop_times.txt"))?;
        // info!("ℹ️ [stop_times] {} records", stop_times.len());
        // self.gtfs.insert_stop_times(&stop_times)?;
        // info!("  ✨ Success");

        let calendars = io::read::<Calendar>(&gtfs_dir.join("calendar.txt"))?;
        info!("ℹ️ [calendar] {} records", calendars.len());
        self.gtfs.insert_calendars(&calendars)?;
        info!("  ✨ Success");

        let calendar_dates = io::read::<CalendarDate>(&gtfs_dir.join("calendar_dates.txt"))?;
        info!("ℹ️ [calendar_dates] {} records", calendar_dates.len());
        self.gtfs.insert_calendar_dates(&calendar_dates)?;
        info!("  ✨ Success");

        let fare_attributes = io::read::<FareAttribute>(&gtfs_dir.join("fare_attributes.txt"))?;
        info!("ℹ️ [fare_attributes] {} records", fare_attributes.len());
        self.gtfs.insert_fare_attributes(&fare_attributes)?;
        info!("  ✨ Success");

        let fare_rules = io::read::<FareRule>(&gtfs_dir.join("fare_rules.txt"))?;
        info!("ℹ️ [fare_rules] {} records", fare_rules.len());
        self.gtfs.insert_fare_rules(&fare_rules)?;
        info!("  ✨ Success");

        // let shapes = io::read::<Shape>(&gtfs_dir.join("shapes.txt"))?;
        // info!("ℹ️ [shapes] {} records", shapes.len());
        // self.gtfs.insert_shapes(&shapes)?;
        // info!("  ✨ Success");

        let frequencies = io::read::<Frequency>(&gtfs_dir.join("frequencies.txt"))?;
        info!("ℹ️ [frequencies] {} records", frequencies.len());
        self.gtfs.insert_frequencies(&frequencies)?;
        info!("  ✨ Success");

        let transfers = io::read::<Transfer>(&gtfs_dir.join("transfers.txt"))?;
        info!("ℹ️ [transfers] {} records", transfers.len());
        self.gtfs.insert_transfers(&transfers)?;
        info!("  ✨ Success");

        let feeds = io::read::<Feed>(&gtfs_dir.join("feed_info.txt"))?;
        info!("ℹ️ [feed_info] {} records", feeds.len());
        self.gtfs.insert_feeds(&feeds)?;
        info!("  ✨ Success");

        Ok(())
    }

    pub fn drop_tables(&mut self) -> Result<()> {
        info!("ℹ️ Drop all tables.");
        self.gtfs.drop_all()?;
        info!("  ✨ Success");
        Ok(())
    }
}
