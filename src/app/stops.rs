use anyhow::Result;

use crate::external::gtfs::stops::Stop;
use crate::external::gtfs::GtfsDbTrait;
use crate::external::gtfsdb::GtfsDb;

pub struct StopServiceDb {
    gtfs: GtfsDb,
}

impl StopServiceDb {
    pub fn new(gtfs: GtfsDb) -> Self {
        Self { gtfs }
    }

    pub fn fetch_stops(&mut self, word: String) -> Result<Vec<Stop>> {
        self.gtfs.select_stops_by_name(word)
    }
}
