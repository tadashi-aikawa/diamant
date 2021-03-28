use anyhow::Result;

use crate::external::gtfs::trips::Trip;
use crate::external::gtfs::GtfsCsvTrait;
use crate::external::gtfscsv::GtfsCsv;

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
