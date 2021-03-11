use anyhow::Result;

use crate::external;
use crate::external::gtfs::trips::Trip;

pub struct TripService {
    gtfs: Box<dyn external::gtfs::Gtfs>,
}

impl TripService {
    pub fn new(gtfs: Box<dyn external::gtfs::Gtfs>) -> Self {
        Self { gtfs }
    }

    pub fn fetch(&mut self) -> Result<Vec<Trip>> {
        self.gtfs.select_trips()
    }
}
