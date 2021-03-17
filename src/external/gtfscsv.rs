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

pub struct GtfsCsv {
    gtfs_dir: PathBuf,
}

pub fn init(path: &Path) -> Result<GtfsCsv> {
    GtfsCsv::new(path)
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

    fn insert_agencies(&mut self, _agencies: &[&Agency]) -> Result<()> {
        unimplemented!()
    }

    fn select_agencies(&mut self) -> Result<Vec<Agency>> {
        let results = io::read::<Agency>(&self.gtfs_dir.join("agency.txt"))?;
        Ok(results)
    }

    fn insert_agencies_jp(&mut self, _agencies_jp: &[&AgencyJp]) -> Result<()> {
        unimplemented!()
    }

    fn select_agencies_jp(&mut self) -> Result<Vec<AgencyJp>> {
        let results = io::read::<AgencyJp>(&self.gtfs_dir.join("agency_jp.txt"))?;
        Ok(results)
    }

    fn insert_stops(&mut self, _stops: &[&Stop]) -> Result<()> {
        unimplemented!()
    }

    fn select_stops(&mut self) -> Result<Vec<Stop>> {
        let results = io::read::<Stop>(&self.gtfs_dir.join("stops.txt"))?;
        Ok(results)
    }

    fn insert_routes(&mut self, _routes: &[&Route]) -> Result<()> {
        unimplemented!()
    }

    fn select_routes(&mut self) -> Result<Vec<Route>> {
        let results = io::read::<Route>(&self.gtfs_dir.join("routes.txt"))?;
        Ok(results)
    }

    fn insert_routes_jp(&mut self, _routes: &[&RouteJp]) -> Result<()> {
        unimplemented!()
    }

    fn select_routes_jp(&mut self) -> Result<Vec<RouteJp>> {
        let results = io::read::<RouteJp>(&self.gtfs_dir.join("routes_jp.txt"))?;
        Ok(results)
    }

    fn insert_trips(&mut self, _trips: &[&Trip]) -> Result<()> {
        unimplemented!()
    }

    fn select_trips(&mut self) -> Result<Vec<Trip>> {
        let results = io::read::<Trip>(&self.gtfs_dir.join("trips.txt"))?;
        Ok(results)
    }

    fn insert_stop_times(&mut self, _stop_times: &[&StopTime]) -> Result<()> {
        unimplemented!()
    }

    fn select_stop_times(&mut self) -> Result<Vec<StopTime>> {
        let results = io::read::<StopTime>(&self.gtfs_dir.join("stop_times.txt"))?;
        Ok(results)
    }

    fn insert_calendars(&mut self, _calendars: &[&Calendar]) -> Result<()> {
        unimplemented!()
    }

    fn select_calendars(&mut self) -> Result<Vec<Calendar>> {
        let results = io::read::<Calendar>(&self.gtfs_dir.join("calendar.txt"))?;
        Ok(results)
    }

    fn insert_calendar_dates(&mut self, _calendar_dates: &[&CalendarDate]) -> Result<()> {
        unimplemented!()
    }

    fn select_calendar_dates(&mut self) -> Result<Vec<CalendarDate>> {
        let results = io::read::<CalendarDate>(&self.gtfs_dir.join("calendar_dates.txt"))?;
        Ok(results)
    }

    fn insert_fare_attributes(&mut self, _fare_attributes: &[&FareAttribute]) -> Result<()> {
        unimplemented!()
    }

    fn select_fare_attributes(&mut self) -> Result<Vec<FareAttribute>> {
        let results = io::read::<FareAttribute>(&self.gtfs_dir.join("fare_attributes.txt"))?;
        Ok(results)
    }

    fn insert_fare_rules(&mut self, _fare_rules: &[&FareRule]) -> Result<()> {
        unimplemented!()
    }

    fn select_fare_rules(&mut self) -> Result<Vec<FareRule>> {
        let results = io::read::<FareRule>(&self.gtfs_dir.join("fare_rules.txt"))?;
        Ok(results)
    }

    fn insert_shapes(&mut self, _shapes: &[&Shape]) -> Result<()> {
        unimplemented!()
    }

    fn select_shapes(&mut self) -> Result<Vec<Shape>> {
        let results = io::read::<Shape>(&self.gtfs_dir.join("shapes.txt"))?;
        Ok(results)
    }

    fn insert_frequencies(&mut self, _frequencies: &[&Frequency]) -> Result<()> {
        unimplemented!()
    }

    fn select_frequencies(&mut self) -> Result<Vec<Frequency>> {
        let results = io::read::<Frequency>(&self.gtfs_dir.join("frequencies.txt"))?;
        Ok(results)
    }

    fn insert_transfers(&mut self, _transfers: &[&Transfer]) -> Result<()> {
        unimplemented!()
    }

    fn select_transfers(&mut self) -> Result<Vec<Transfer>> {
        let results = io::read::<Transfer>(&self.gtfs_dir.join("transfers.txt"))?;
        Ok(results)
    }

    fn insert_feeds(&mut self, _feeds: &[&Feed]) -> Result<()> {
        unimplemented!()
    }

    fn select_feeds(&mut self) -> Result<Vec<Feed>> {
        let results = io::read::<Feed>(&self.gtfs_dir.join("feed_info.txt"))?;
        Ok(results)
    }

    fn insert_translations(&mut self, _translations: &[&Translation]) -> Result<()> {
        unimplemented!()
    }

    fn select_translations(&mut self) -> Result<Vec<Translation>> {
        let results = io::read::<Translation>(&self.gtfs_dir.join("translations.txt"))?;
        Ok(results)
    }
}
