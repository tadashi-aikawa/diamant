use anyhow::Result;

use crate::external::gtfs::stops::StopId;
use crate::external::gtfs::trips::Trip;
use crate::external::gtfs::{GtfsCsvTrait, GtfsDbTrait};
use crate::external::gtfscsv::GtfsCsv;
use crate::external::gtfsdb::GtfsDb;

pub trait TripService {
    fn fetch(&mut self) -> Result<Vec<Trip>>;
}

pub struct TripServiceCsv {
    gtfs: GtfsCsv,
}

impl TripServiceCsv {
    pub fn new(gtfs: GtfsCsv) -> Self {
        Self { gtfs }
    }
}

impl TripService for TripServiceCsv {
    fn fetch(&mut self) -> Result<Vec<Trip>> {
        self.gtfs.load_trips()
    }
}

pub struct TripServiceDb {
    gtfs: GtfsDb,
}

impl TripServiceDb {
    pub fn new(gtfs: GtfsDb) -> Self {
        Self { gtfs }
    }

    pub fn fetch_trips(&mut self, stop_id: StopId) -> Result<Vec<Trip>> {
        self.gtfs.select_trips(stop_id)
    }
}
