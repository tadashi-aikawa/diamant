use anyhow::Result;

use crate::external::gtfs::routes::Route;
use crate::external::gtfs::GtfsCsvTrait;
use crate::external::gtfscsv::GtfsCsv;

pub trait RouteService {
    fn fetch(&mut self) -> Result<Vec<Route>>;
}

pub struct RouteServiceCsv {
    gtfs: GtfsCsv,
}

impl RouteServiceCsv {
    pub fn new(gtfs: GtfsCsv) -> Self {
        Self { gtfs }
    }
}

impl RouteService for RouteServiceCsv {
    fn fetch(&mut self) -> Result<Vec<Route>> {
        self.gtfs.load_routes()
    }
}
