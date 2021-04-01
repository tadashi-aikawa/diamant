use anyhow::Result;

use crate::external::gtfs::routes::Route;
use crate::external::gtfs::{GtfsCsvTrait, GtfsDbTrait};
use crate::external::gtfscsv::GtfsCsv;
use crate::external::gtfsdb::GtfsDb;

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

pub struct RouteServiceDb {
    gtfs: GtfsDb,
}

impl RouteServiceDb {
    pub fn new(gtfs: GtfsDb) -> Self {
        Self { gtfs }
    }
}

impl RouteService for RouteServiceDb {
    fn fetch(&mut self) -> Result<Vec<Route>> {
        self.gtfs.select_routes()
    }
}
