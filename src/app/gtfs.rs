use anyhow::Result;
use itertools::Itertools;
use log::info;

use crate::external;
use std::path::PathBuf;

pub struct GtfsService<G1, G2>
where
    G1: external::gtfs::Gtfs,
    G2: external::gtfs::Gtfs,
{
    gtfs_db: G1,
    gtfs_csv: G2,
}

/// GTFS全体を横断するアプリケーションサービス
impl<G1, G2> GtfsService<G1, G2>
where
    G1: external::gtfs::Gtfs,
    G2: external::gtfs::Gtfs,
{
    pub fn new(gtfs_db: G1, gtfs_csv: G2) -> Self {
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
        let agencies = agencies.iter().unique().collect_vec();
        info!("ℹ️ [agencies] {} records", agencies.len());
        self.gtfs_db.insert_agencies(&agencies)?;
        info!("  ✨ Success");

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("agency_jp.txt");
        if path.exists() {
            let agencies_jp = self.gtfs_csv.select_agencies_jp()?;
            let agencies_jp = agencies_jp.iter().unique().collect_vec();
            info!("ℹ️ [agency_jp] {} records", agencies_jp.len());
            self.gtfs_db.insert_agencies_jp(&agencies_jp)?;
            info!("  ✨ Success");
        } else {
            info!("ℹ️ [agency_jp] Skip because agency_jp.txt was not found");
        }

        let stops = self.gtfs_csv.select_stops()?;
        let stops = stops.iter().unique().collect_vec();
        info!("ℹ️ [stops] {} records", stops.len());
        self.gtfs_db.insert_stops(&stops)?;
        info!("  ✨ Success");

        let routes = self.gtfs_csv.select_routes()?;
        let routes = routes.iter().unique().collect_vec();
        info!("ℹ️ [routes] {} records", routes.len());
        self.gtfs_db.insert_routes(&routes)?;
        info!("  ✨ Success");

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("routes_jp.txt");
        if path.exists() {
            let routes_jp = self.gtfs_csv.select_routes_jp()?;
            let routes_jp = routes_jp.iter().unique().collect_vec();
            info!("ℹ️ [routes_jp] {} records", routes_jp.len());
            self.gtfs_db.insert_routes_jp(&routes_jp)?;
            info!("  ✨ Success");
        } else {
            info!("ℹ️ [routes_jp] Skip because routes_jp.txt was not found");
        }

        let trips = self.gtfs_csv.select_trips()?;
        let trips = trips.iter().unique().collect_vec();
        info!("ℹ️ [trips] {} records", trips.len());
        self.gtfs_db.insert_trips(&trips)?;
        info!("  ✨ Success");

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("office_jp.txt");
        if path.exists() {
            let offices_jp = self.gtfs_csv.select_offices_jp()?;
            let offices_jp = offices_jp.iter().unique().collect_vec();
            info!("ℹ️ [office_jp] {} records", offices_jp.len());
            self.gtfs_db.insert_offices_jp(&offices_jp)?;
            info!("  ✨ Success");
        } else {
            info!("ℹ️ [office_jp] Skip because office_jp.txt was not found");
        }

        let stop_times = self.gtfs_csv.select_stop_times()?;
        let stop_times = stop_times.iter().unique().collect_vec();
        info!("ℹ️ [stop_times] {} records", stop_times.len());
        self.gtfs_db.insert_stop_times(&stop_times)?;
        info!("  ✨ Success");

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("calendar.txt");
        if path.exists() {
            let calendars = self.gtfs_csv.select_calendars()?;
            let calendars = calendars.iter().unique().collect_vec();
            info!("ℹ️ [calendar] {} records", calendars.len());
            self.gtfs_db.insert_calendars(&calendars)?;
            info!("  ✨ Success");
        }

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("calendar_dates.txt");
        if path.exists() {
            let calendar_dates = self.gtfs_csv.select_calendar_dates()?;
            let calendar_dates = calendar_dates.iter().unique().collect_vec();
            info!("ℹ️ [calendar_dates] {} records", calendar_dates.len());
            self.gtfs_db.insert_calendar_dates(&calendar_dates)?;
            info!("  ✨ Success");
        }

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("fare_attributes.txt");
        if path.exists() {
            let fare_attributes = self.gtfs_csv.select_fare_attributes()?;
            let fare_attributes = fare_attributes.iter().unique().collect_vec();
            info!("ℹ️ [fare_attributes] {} records", fare_attributes.len());
            self.gtfs_db.insert_fare_attributes(&fare_attributes)?;
            info!("  ✨ Success");
        }

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("fare_rules.txt");
        if path.exists() {
            let fare_rules = self.gtfs_csv.select_fare_rules()?;
            let fare_rules = fare_rules.iter().unique().collect_vec();
            info!("ℹ️ [fare_rules] {} records", fare_rules.len());
            self.gtfs_db.insert_fare_rules(&fare_rules)?;
            info!("  ✨ Success");
        }

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("shapes.txt");
        if path.exists() {
            let shapes = self.gtfs_csv.select_shapes()?;
            let shapes = shapes.iter().unique().collect_vec();
            info!("ℹ️ [shapes] {} records", shapes.len());
            self.gtfs_db.insert_shapes(&shapes)?;
            info!("  ✨ Success");
        }

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("frequencies.txt");
        if path.exists() {
            let frequencies = self.gtfs_csv.select_frequencies()?;
            let frequencies = frequencies.iter().unique().collect_vec();
            info!("ℹ️ [frequencies] {} records", frequencies.len());
            self.gtfs_db.insert_frequencies(&frequencies)?;
            info!("  ✨ Success");
        }

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("transfers.txt");
        if path.exists() {
            let transfers = self.gtfs_csv.select_transfers()?;
            let transfers = transfers.iter().unique().collect_vec();
            info!("ℹ️ [transfers] {} records", transfers.len());
            self.gtfs_db.insert_transfers(&transfers)?;
            info!("  ✨ Success");
        }

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("feed_info.txt");
        if path.exists() {
            let feeds = self.gtfs_csv.select_feeds()?;
            let feeds = feeds.iter().unique().collect_vec();
            info!("ℹ️ [feed_info] {} records", feeds.len());
            self.gtfs_db.insert_feeds(&feeds)?;
            info!("  ✨ Success");
        }

        // TODO: 判定処理をgtfsのIFで
        let path = gtfs_dir.join("translations.txt");
        if path.exists() {
            let translations = self.gtfs_csv.select_translations()?;
            let translations = translations.iter().unique().collect_vec();
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
