use std::path::{Path, PathBuf};

use anyhow::Result;
use serde::de::DeserializeOwned;

use crate::external::gtfs::agency::Agency;
use crate::external::gtfs::agency_jp::AgencyJp;
use crate::external::gtfs::calendar::Calendar;
use crate::external::gtfs::calendar_dates::CalendarDate;
use crate::external::gtfs::fare_attributes::FareAttribute;
use crate::external::gtfs::fare_rules::FareRule;
use crate::external::gtfs::feed_info::Feed;
use crate::external::gtfs::frequencies::Frequency;
use crate::external::gtfs::legacy_translations::LegacyTranslation;
use crate::external::gtfs::office_jp::OfficeJp;
use crate::external::gtfs::routes::Route;
use crate::external::gtfs::routes_jp::RouteJp;
use crate::external::gtfs::shapes::Shape;
use crate::external::gtfs::stop_times::StopTime;
use crate::external::gtfs::stops::Stop;
use crate::external::gtfs::transfers::Transfer;
use crate::external::gtfs::translations::Translation;
use crate::external::gtfs::trips::Trip;
use crate::external::gtfs::GtfsCsvTrait;
use crate::io;
use crate::io::Format;

pub struct GtfsCsv {
    gtfs_dir: PathBuf,
}

pub trait GTFSFile {
    fn file_name() -> &'static str;
}

fn load_gtfs<T>(gtfs_dir: &Path) -> Result<Vec<T>>
where
    T: GTFSFile + DeserializeOwned,
{
    io::read::<T>(&gtfs_dir.join(T::file_name()), &Format::Csv)
}

fn has_gtfs<T>(gtfs_dir: &Path) -> bool
where
    T: GTFSFile,
{
    gtfs_dir.join(T::file_name()).exists()
}

impl GtfsCsv {
    pub fn new(gtfs_dir: &Path) -> Result<Self> {
        Ok(GtfsCsv {
            gtfs_dir: gtfs_dir.into(),
        })
    }
}

impl GtfsCsvTrait for GtfsCsv {
    fn load_agencies(&mut self) -> Result<Vec<Agency>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn load_agencies_jp(&mut self) -> Result<Vec<AgencyJp>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_agency_jp(&mut self) -> bool {
        has_gtfs::<AgencyJp>(&self.gtfs_dir)
    }

    fn load_stops(&mut self) -> Result<Vec<Stop>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn load_routes(&mut self) -> Result<Vec<Route>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn load_routes_jp(&mut self) -> Result<Vec<RouteJp>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_routes_jp(&mut self) -> bool {
        has_gtfs::<RouteJp>(&self.gtfs_dir)
    }

    fn load_trips(&mut self) -> Result<Vec<Trip>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn load_offices_jp(&mut self) -> Result<Vec<OfficeJp>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_office_jp(&mut self) -> bool {
        has_gtfs::<OfficeJp>(&self.gtfs_dir)
    }

    fn load_stop_times(&mut self) -> Result<Vec<StopTime>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn load_calendars(&mut self) -> Result<Vec<Calendar>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn load_calendar_dates(&mut self) -> Result<Vec<CalendarDate>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_calendar_dates(&mut self) -> bool {
        has_gtfs::<CalendarDate>(&self.gtfs_dir)
    }

    fn load_fare_attributes(&mut self) -> Result<Vec<FareAttribute>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_fare_attributes(&mut self) -> bool {
        has_gtfs::<FareAttribute>(&self.gtfs_dir)
    }

    fn load_fare_rules(&mut self) -> Result<Vec<FareRule>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_fare_rules(&mut self) -> bool {
        has_gtfs::<FareRule>(&self.gtfs_dir)
    }

    fn select_shapes(&mut self) -> Result<Vec<Shape>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_shapes(&mut self) -> bool {
        has_gtfs::<Shape>(&self.gtfs_dir)
    }

    fn load_frequencies(&mut self) -> Result<Vec<Frequency>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_frequencies(&mut self) -> bool {
        has_gtfs::<Frequency>(&self.gtfs_dir)
    }

    fn load_transfers(&mut self) -> Result<Vec<Transfer>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_transfers(&mut self) -> bool {
        has_gtfs::<Transfer>(&self.gtfs_dir)
    }

    fn load_feeds(&mut self) -> Result<Vec<Feed>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn load_translations(&mut self) -> Result<Vec<Translation>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn load_legacy_translations(&mut self) -> Result<Vec<LegacyTranslation>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }
}
