use anyhow::Result;

use crate::external::gtfs::extended::stop_time_details::StopTimeDetail;
use crate::external::gtfs::trips::TripId;
use crate::external::gtfs::GtfsDbTrait;
use crate::external::gtfsdb::GtfsDb;

pub struct StopTimeServiceDb {
    gtfs: GtfsDb,
}

impl StopTimeServiceDb {
    pub fn new(gtfs: GtfsDb) -> Self {
        Self { gtfs }
    }

    pub fn fetch_stop_time_details(
        &mut self,
        trip_ids: Option<Vec<TripId>>,
        stop_name_prefix: Option<String>,
    ) -> Result<Vec<StopTimeDetail>> {
        self.gtfs
            .select_stop_time_details(trip_ids, stop_name_prefix)
    }
}
