use anyhow::Result;
use itertools::Itertools;
use log::info;

use crate::external;
use crate::external::gtfs::translations::Translation;

pub struct GtfsService<CSV, DB>
where
    CSV: external::gtfs::GtfsCsvTrait,
    DB: external::gtfs::GtfsDbTrait,
{
    gtfs_csv: CSV,
    gtfs_db: DB,
}

/// GTFS全体を横断するアプリケーションサービス
impl<CSV, DB> GtfsService<CSV, DB>
where
    CSV: external::gtfs::GtfsCsvTrait,
    DB: external::gtfs::GtfsDbTrait,
{
    pub fn new(gtfs_csv: CSV, gtfs_db: DB) -> Self {
        Self { gtfs_csv, gtfs_db }
    }

    pub fn create_tables(&mut self) -> Result<()> {
        info!("ℹ️ Create all tables.");
        self.gtfs_db.create_all()?;
        info!("  ✨ Success");
        Ok(())
    }

    pub fn insert_tables(&mut self, legacy_translations: bool) -> Result<()> {
        // translationのlegacyフラグが間違っている場合に失敗するため最初に実行
        if legacy_translations {
            // 昔のtranslation
            info!("ℹ️ [translations] Load legacy translations");
            let translations = self.gtfs_csv.load_legacy_translations()?;
            let translations = translations
                .iter()
                .unique()
                .flat_map(Translation::from_legacy)
                .collect_vec();
            info!("ℹ️ [translations] {} records", translations.len());
            self.gtfs_db.insert_translations(&translations)?;
            info!("  ✨ Success");
        } else {
            let translations = self.gtfs_csv.load_translations()?;
            let translations = translations.into_iter().unique().collect_vec();
            info!("ℹ️ [translations] {} records", translations.len());
            self.gtfs_db.insert_translations(&translations)?;
            info!("  ✨ Success");
        }

        let agencies = self.gtfs_csv.load_agencies()?;
        let agencies = agencies.into_iter().unique().collect_vec();
        info!("ℹ️ [agencies] {} records", agencies.len());
        self.gtfs_db.insert_agencies(&agencies)?;
        info!("  ✨ Success");

        if self.gtfs_csv.has_agency_jp() {
            let agencies_jp = self.gtfs_csv.load_agencies_jp()?;
            let agencies_jp = agencies_jp.into_iter().unique().collect_vec();
            info!("ℹ️ [agency_jp] {} records", agencies_jp.len());
            self.gtfs_db.insert_agencies_jp(&agencies_jp)?;
            info!("  ✨ Success");
        } else {
            info!("ℹ️ [agency_jp] Skip because agency_jp.txt was not found");
        }

        if self.gtfs_csv.has_office_jp() {
            let offices_jp = self.gtfs_csv.load_offices_jp()?;
            let offices_jp = offices_jp.into_iter().unique().collect_vec();
            info!("ℹ️ [office_jp] {} records", offices_jp.len());
            self.gtfs_db.insert_offices_jp(&offices_jp)?;
            info!("  ✨ Success");
        } else {
            info!("ℹ️ [office_jp] Skip because office_jp.txt was not found");
        }

        let calendars = self.gtfs_csv.load_calendars()?;
        let calendars = calendars.into_iter().unique().collect_vec();
        info!("ℹ️ [calendar] {} records", calendars.len());
        self.gtfs_db.insert_calendars(&calendars)?;
        info!("  ✨ Success");

        if self.gtfs_csv.has_calendar_dates() {
            let calendar_dates = self.gtfs_csv.load_calendar_dates()?;
            let calendar_dates = calendar_dates.into_iter().unique().collect_vec();
            info!("ℹ️ [calendar_dates] {} records", calendar_dates.len());
            self.gtfs_db.insert_calendar_dates(&calendar_dates)?;
            info!("  ✨ Success");
        } else {
            info!("ℹ️ [calendar_dates] Skip because calendar_dates.txt was not found");
        }

        let stops = self.gtfs_csv.load_stops()?;
        let stops = stops.into_iter().unique().collect_vec();
        info!("ℹ️ [stops] {} records", stops.len());
        self.gtfs_db.insert_stops(&stops)?;
        info!("  ✨ Success");

        let routes = self.gtfs_csv.load_routes()?;
        let routes = routes.into_iter().unique().collect_vec();
        info!("ℹ️ [routes] {} records", routes.len());
        self.gtfs_db.insert_routes(&routes)?;
        info!("  ✨ Success");

        if self.gtfs_csv.has_routes_jp() {
            let routes_jp = self.gtfs_csv.load_routes_jp()?;
            let routes_jp = routes_jp.into_iter().unique().collect_vec();
            info!("ℹ️ [routes_jp] {} records", routes_jp.len());
            self.gtfs_db.insert_routes_jp(&routes_jp)?;
            info!("  ✨ Success");
        } else {
            info!("ℹ️ [routes_jp] Skip because routes_jp.txt was not found");
        }

        let trips = self.gtfs_csv.load_trips()?;
        let trips = trips.into_iter().unique().collect_vec();
        info!("ℹ️ [trips] {} records", trips.len());
        self.gtfs_db.insert_trips(&trips)?;
        info!("  ✨ Success");

        let stop_times = self.gtfs_csv.load_stop_times()?;
        let stop_times = stop_times.into_iter().unique().collect_vec();
        info!("ℹ️ [stop_times] {} records", stop_times.len());
        self.gtfs_db.insert_stop_times(&stop_times)?;
        info!("  ✨ Success");

        if self.gtfs_csv.has_fare_attributes() {
            let fare_attributes = self.gtfs_csv.load_fare_attributes()?;
            let fare_attributes = fare_attributes.into_iter().unique().collect_vec();
            info!("ℹ️ [fare_attributes] {} records", fare_attributes.len());
            self.gtfs_db.insert_fare_attributes(&fare_attributes)?;
            info!("  ✨ Success");
        } else {
            info!("ℹ️ [fare_attributes] Skip because fare_attributes.txt was not found");
        }

        if self.gtfs_csv.has_fare_rules() {
            let fare_rules = self.gtfs_csv.load_fare_rules()?;
            let fare_rules = fare_rules.into_iter().unique().collect_vec();
            info!("ℹ️ [fare_rules] {} records", fare_rules.len());
            self.gtfs_db.insert_fare_rules(&fare_rules)?;
            info!("  ✨ Success");
        } else {
            info!("ℹ️ [fare_rules] Skip because fare_rules.txt was not found");
        }

        if self.gtfs_csv.has_shapes() {
            let shapes = self.gtfs_csv.select_shapes()?;
            let shapes = shapes.into_iter().unique().collect_vec();
            info!("ℹ️ [shapes] {} records", shapes.len());
            self.gtfs_db.insert_shapes(&shapes)?;
            info!("  ✨ Success");
        } else {
            info!("ℹ️ [shapes] Skip because shapes.txt was not found");
        }

        if self.gtfs_csv.has_frequencies() {
            let frequencies = self.gtfs_csv.load_frequencies()?;
            let frequencies = frequencies.into_iter().unique().collect_vec();
            info!("ℹ️ [frequencies] {} records", frequencies.len());
            self.gtfs_db.insert_frequencies(&frequencies)?;
            info!("  ✨ Success");
        } else {
            info!("ℹ️ [frequencies] Skip because frequencies.txt was not found");
        }

        if self.gtfs_csv.has_transfers() {
            let transfers = self.gtfs_csv.load_transfers()?;
            let transfers = transfers.into_iter().unique().collect_vec();
            info!("ℹ️ [transfers] {} records", transfers.len());
            self.gtfs_db.insert_transfers(&transfers)?;
            info!("  ✨ Success");
        } else {
            info!("ℹ️ [transfers] Skip because transfers.txt was not found");
        }

        // GTFS-JPでは必須
        let feeds = self.gtfs_csv.load_feeds()?;
        let feeds = feeds.into_iter().unique().collect_vec();
        info!("ℹ️ [feed_info] {} records", feeds.len());
        self.gtfs_db.insert_feeds(&feeds)?;
        info!("  ✨ Success");

        Ok(())
    }

    pub fn drop_tables(&mut self) -> Result<()> {
        info!("ℹ️ Drop all tables.");
        self.gtfs_db.drop_all()?;
        info!("  ✨ Success");
        Ok(())
    }
}
