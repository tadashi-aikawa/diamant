use std::path::{Path, PathBuf};

use anyhow::Result;

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
use crate::external::gtfs::Gtfs;
use crate::io;
use serde::de::DeserializeOwned;

pub struct GtfsCsv {
    gtfs_dir: PathBuf,
}

pub trait GTFSFile {
    fn file_name() -> &'static str;
}

pub fn init(path: &Path) -> Result<GtfsCsv> {
    GtfsCsv::new(path)
}

fn load_gtfs<T>(gtfs_dir: &Path) -> Result<Vec<T>>
where
    T: GTFSFile + DeserializeOwned,
{
    io::read::<T>(&gtfs_dir.join(T::file_name()))
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

impl Gtfs for GtfsCsv {
    fn create_all(&self) -> Result<()> {
        unimplemented!()
    }

    fn drop_all(&self) -> Result<()> {
        unimplemented!()
    }

    fn insert_agencies(&mut self, _agencies: &[Agency]) -> Result<()> {
        unimplemented!()
    }

    fn select_agencies(&mut self) -> Result<Vec<Agency>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn insert_agencies_jp(&mut self, _agencies_jp: &[AgencyJp]) -> Result<()> {
        unimplemented!()
    }

    fn select_agencies_jp(&mut self) -> Result<Vec<AgencyJp>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_agency_jp(&mut self) -> bool {
        has_gtfs::<AgencyJp>(&self.gtfs_dir)
    }

    fn insert_stops(&mut self, _stops: &[Stop]) -> Result<()> {
        unimplemented!()
    }

    fn select_stops(&mut self) -> Result<Vec<Stop>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn insert_routes(&mut self, _routes: &[Route]) -> Result<()> {
        unimplemented!()
    }

    fn select_routes(&mut self) -> Result<Vec<Route>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn insert_routes_jp(&mut self, _routes: &[RouteJp]) -> Result<()> {
        unimplemented!()
    }

    fn select_routes_jp(&mut self) -> Result<Vec<RouteJp>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_routes_jp(&mut self) -> bool {
        has_gtfs::<RouteJp>(&self.gtfs_dir)
    }

    fn insert_trips(&mut self, _trips: &[Trip]) -> Result<()> {
        unimplemented!()
    }

    fn select_trips(&mut self) -> Result<Vec<Trip>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn insert_offices_jp(&mut self, _offices: &[OfficeJp]) -> Result<()> {
        unimplemented!()
    }

    fn select_offices_jp(&mut self) -> Result<Vec<OfficeJp>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_office_jp(&mut self) -> bool {
        has_gtfs::<OfficeJp>(&self.gtfs_dir)
    }

    fn insert_stop_times(&mut self, _stop_times: &[StopTime]) -> Result<()> {
        unimplemented!()
    }

    fn select_stop_times(&mut self) -> Result<Vec<StopTime>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn insert_calendars(&mut self, _calendars: &[Calendar]) -> Result<()> {
        unimplemented!()
    }

    fn select_calendars(&mut self) -> Result<Vec<Calendar>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn insert_calendar_dates(&mut self, _calendar_dates: &[CalendarDate]) -> Result<()> {
        unimplemented!()
    }

    fn select_calendar_dates(&mut self) -> Result<Vec<CalendarDate>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_calendar_dates(&mut self) -> bool {
        has_gtfs::<CalendarDate>(&self.gtfs_dir)
    }

    fn insert_fare_attributes(&mut self, _fare_attributes: &[FareAttribute]) -> Result<()> {
        unimplemented!()
    }

    fn select_fare_attributes(&mut self) -> Result<Vec<FareAttribute>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_fare_attributes(&mut self) -> bool {
        has_gtfs::<FareAttribute>(&self.gtfs_dir)
    }

    fn insert_fare_rules(&mut self, _fare_rules: &[FareRule]) -> Result<()> {
        unimplemented!()
    }

    fn select_fare_rules(&mut self) -> Result<Vec<FareRule>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_fare_rules(&mut self) -> bool {
        has_gtfs::<FareRule>(&self.gtfs_dir)
    }

    fn insert_shapes(&mut self, _shapes: &[Shape]) -> Result<()> {
        unimplemented!()
    }

    fn select_shapes(&mut self) -> Result<Vec<Shape>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_shapes(&mut self) -> bool {
        has_gtfs::<Shape>(&self.gtfs_dir)
    }

    fn insert_frequencies(&mut self, _frequencies: &[Frequency]) -> Result<()> {
        unimplemented!()
    }

    fn select_frequencies(&mut self) -> Result<Vec<Frequency>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_frequencies(&mut self) -> bool {
        has_gtfs::<Frequency>(&self.gtfs_dir)
    }

    fn insert_transfers(&mut self, _transfers: &[Transfer]) -> Result<()> {
        unimplemented!()
    }

    fn select_transfers(&mut self) -> Result<Vec<Transfer>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn has_transfers(&mut self) -> bool {
        has_gtfs::<Transfer>(&self.gtfs_dir)
    }

    fn insert_feeds(&mut self, _feeds: &[Feed]) -> Result<()> {
        unimplemented!()
    }

    fn select_feeds(&mut self) -> Result<Vec<Feed>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn insert_translations(&mut self, _translations: &[Translation]) -> Result<()> {
        unimplemented!()
    }

    fn select_translations(&mut self) -> Result<Vec<Translation>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }

    fn insert_legacy_translations(&mut self, _translations: &[LegacyTranslation]) -> Result<()> {
        unimplemented!()
    }

    fn select_legacy_translations(&mut self) -> Result<Vec<LegacyTranslation>> {
        load_gtfs::<_>(&self.gtfs_dir)
    }
}
