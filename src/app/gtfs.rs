use anyhow::Result;
use log::info;

use crate::external;
use std::path::PathBuf;

pub struct GtfsService {
    gtfs_db: Box<dyn external::gtfs::Gtfs>,
    gtfs_csv: Box<dyn external::gtfs::Gtfs>,
}

/// GTFS全体を横断するアプリケーションサービス
impl GtfsService {
    pub fn new(
        gtfs_db: Box<dyn external::gtfs::Gtfs>,
        gtfs_csv: Box<dyn external::gtfs::Gtfs>,
    ) -> Self {
        Self { gtfs_db, gtfs_csv }
    }

    pub fn create_tables(&mut self) -> Result<()> {
        info!("ℹ️ Create all tables.");
        self.gtfs_db.create_all()?;
        info!("  ✨ Success");
        Ok(())
    }

    pub fn insert_tables(&mut self, gtfs_dir: &PathBuf) -> Result<()> {
        let agencies = self.gtfs_csv.select_agencies()?;
        info!("ℹ️ [agencies] {} records", agencies.len());
        self.gtfs_db.insert_agencies(&agencies)?;
        info!("  ✨ Success");

        let stops = self.gtfs_csv.select_stops()?;
        info!("ℹ️ [stops] {} records", stops.len());
        self.gtfs_db.insert_stops(&stops)?;
        info!("  ✨ Success");

        let routes = self.gtfs_csv.select_routes()?;
        info!("ℹ️ [routes] {} records", routes.len());
        self.gtfs_db.insert_routes(&routes)?;
        info!("  ✨ Success");

        let trips = self.gtfs_csv.select_trips()?;
        info!("ℹ️ [trips] {} records", trips.len());
        self.gtfs_db.insert_trips(&trips)?;
        info!("  ✨ Success");

        let stop_times = self.gtfs_csv.select_stop_times()?;
        info!("ℹ️ [stop_times] {} records", stop_times.len());
        self.gtfs_db.insert_stop_times(&stop_times)?;
        info!("  ✨ Success");

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("calendar.txt");
        if path.exists() {
            let calendars = self.gtfs_csv.select_calendars()?;
            info!("ℹ️ [calendar] {} records", calendars.len());
            self.gtfs_db.insert_calendars(&calendars)?;
            info!("  ✨ Success");
        }

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("calendar_dates.txt");
        if path.exists() {
            let calendar_dates = self.gtfs_csv.select_calendar_dates()?;
            info!("ℹ️ [calendar_dates] {} records", calendar_dates.len());
            self.gtfs_db.insert_calendar_dates(&calendar_dates)?;
            info!("  ✨ Success");
        }

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("fare_attributes.txt");
        if path.exists() {
            let fare_attributes = self.gtfs_csv.select_fare_attributes()?;
            info!("ℹ️ [fare_attributes] {} records", fare_attributes.len());
            self.gtfs_db.insert_fare_attributes(&fare_attributes)?;
            info!("  ✨ Success");
        }

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("fare_rules.txt");
        if path.exists() {
            let fare_rules = self.gtfs_csv.select_fare_rules()?;
            info!("ℹ️ [fare_rules] {} records", fare_rules.len());
            self.gtfs_db.insert_fare_rules(&fare_rules)?;
            info!("  ✨ Success");
        }

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("shapes.txt");
        if path.exists() {
            let shapes = self.gtfs_csv.select_shapes()?;
            info!("ℹ️ [shapes] {} records", shapes.len());
            self.gtfs_db.insert_shapes(&shapes)?;
            info!("  ✨ Success");
        }

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("frequencies.txt");
        if path.exists() {
            let frequencies = self.gtfs_csv.select_frequencies()?;
            info!("ℹ️ [frequencies] {} records", frequencies.len());
            self.gtfs_db.insert_frequencies(&frequencies)?;
            info!("  ✨ Success");
        }

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("transfers.txt");
        if path.exists() {
            let transfers = self.gtfs_csv.select_transfers()?;
            info!("ℹ️ [transfers] {} records", transfers.len());
            self.gtfs_db.insert_transfers(&transfers)?;
            info!("  ✨ Success");
        }

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("feed_info.txt");
        if path.exists() {
            let feeds = self.gtfs_csv.select_feeds()?;
            info!("ℹ️ [feed_info] {} records", feeds.len());
            self.gtfs_db.insert_feeds(&feeds)?;
            info!("  ✨ Success");
        }

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("translations.txt");
        if path.exists() {
            let translations = self.gtfs_csv.select_translations()?;
            info!("ℹ️ [translations] {} records", translations.len());
            self.gtfs_db.insert_translations(&translations)?;
            info!("  ✨ Success");
        }

        Ok(())
    }

    pub fn drop_tables(&mut self) -> Result<()> {
        info!("ℹ️ Drop all tables.");
        self.gtfs_db.drop_all()?;
        info!("  ✨ Success");
        Ok(())
    }
}
