use anyhow::Result;

use crate::external::gtfs::extended::service_route_identity::ServiceRouteIdentity;
use crate::external::gtfs::GtfsDbTrait;
use crate::external::gtfsdb::GtfsDb;

pub struct ServiceRouteServiceDb {
    gtfs: GtfsDb,
}

impl ServiceRouteServiceDb {
    pub fn new(gtfs: GtfsDb) -> Self {
        Self { gtfs }
    }

    pub fn fetch_service_route_identity(&mut self) -> Result<Vec<ServiceRouteIdentity>> {
        self.gtfs.select_service_route_identity()
    }
}
