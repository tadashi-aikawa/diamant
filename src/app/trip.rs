use anyhow::Result;

use crate::external;
use crate::external::gtfs::trips::Trip;

pub struct TripService<G>
where
    G: external::gtfs::Gtfs,
{
    gtfs: G,
}

impl<G> TripService<G>
where
    G: external::gtfs::Gtfs,
{
    pub fn new(gtfs: G) -> Self {
        Self { gtfs }
    }

    pub fn fetch(&mut self) -> Result<Vec<Trip>> {
        self.gtfs.select_trips()
    }
}
