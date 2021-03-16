use anyhow::Result;

use crate::external;
use crate::external::gtfs::routes::Route;

pub struct RouteService<G>
where
    G: external::gtfs::Gtfs,
{
    gtfs: G,
}

impl<G> RouteService<G>
where
    G: external::gtfs::Gtfs,
{
    pub fn new(gtfs: G) -> Self {
        Self { gtfs }
    }

    pub fn fetch(&mut self) -> Result<Vec<Route>> {
        self.gtfs.select_routes()
    }
}
