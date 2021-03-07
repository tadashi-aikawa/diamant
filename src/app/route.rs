use anyhow::Result;

use crate::external;
use crate::external::gtfs::routes::{Route, RouteId};

pub struct RouteService {
    gtfs: Box<dyn external::gtfs::Gtfs>,
}

impl RouteService {
    pub fn new(gtfs: Box<dyn external::gtfs::Gtfs>) -> Self {
        Self { gtfs }
    }

    pub fn fetch(&mut self, route_id: &Option<RouteId>) -> Result<Vec<Route>> {
        self.gtfs.select_routes(route_id)
    }
}
