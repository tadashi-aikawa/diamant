use crate::external::gtfs::extended::stop_time_details::StopTimeDetail;
use crate::external::gtfs::Direction;
use crate::external::gtfsdb::Table;
use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{EnumString, EnumVariantNames};

/// サービスルートID (ex: 1)
pub type ServiceRouteId = i32;
/// サービスルートフルID (ex: 1^1)
pub type ServiceRouteFullId = String;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct ServiceRoute {
    /// サービスルートID
    pub service_route_id: ServiceRouteId,
    /// サービスルート名
    pub service_route_name: String,
    /// 上下区分
    pub direction_id: Direction,
}

impl ServiceRoute {
    fn full_id(self) -> ServiceRouteFullId {
        format!(
            "{}^{}",
            self.service_route_id,
            serde_json::to_string(&self.direction_id).unwrap()
        )
    }
}

impl Table for ServiceRoute {
    fn table_name() -> &'static str {
        "service_routes"
    }

    fn column_names() -> &'static [&'static str] {
        &["service_route_id", "service_route_name", "direction_id"]
    }

    fn create_sql() -> &'static str {
        "
        service_route_id int,
        service_route_name text,
        direction_id int,
        PRIMARY KEY(service_route_id, direction_id)
        "
    }
}

/// 同一性
type Identify = String;

#[derive(Debug, Clone, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum IdentifyStrategy {
    StopIds,
    StopNames,
    RouteId,
    RouteShortName,
    RouteLongName,
}

pub struct ServiceRouteGenerator {
    service_route_id: ServiceRouteId,
    pub service_route_by_identify: HashMap<Identify, ServiceRoute>,
    identify_strategy: IdentifyStrategy,
}

impl ServiceRouteGenerator {
    pub fn new(strategy: &IdentifyStrategy) -> Self {
        ServiceRouteGenerator {
            service_route_id: 0,
            service_route_by_identify: HashMap::new(),
            identify_strategy: strategy.clone(),
        }
    }

    fn to_identify(&self, stop_time_details: &[StopTimeDetail]) -> Result<String> {
        let first_stop = stop_time_details.first().unwrap();
        match self.identify_strategy {
            IdentifyStrategy::StopIds => Ok(stop_time_details
                .iter()
                .map(|x| x.stop_id.clone())
                .join(",")),
            IdentifyStrategy::StopNames => Ok(stop_time_details
                .iter()
                .map(|x| x.stop_name.clone())
                .join(",")),
            IdentifyStrategy::RouteId => Ok(first_stop.route_id.clone()),
            IdentifyStrategy::RouteShortName => {
                first_stop.route_short_name.clone().ok_or_else(|| {
                    anyhow!(
                        "route_short_nameが空です. route_id = {}",
                        first_stop.route_id
                    )
                })
            }
            IdentifyStrategy::RouteLongName => {
                first_stop.route_long_name.clone().ok_or_else(|| {
                    anyhow!(
                        "route_long_nameが空です. route_id = {}",
                        first_stop.route_id
                    )
                })
            }
        }
    }

    /// stop_time_detailsは 1つのtripに対し、sequence昇順
    pub fn generate(&mut self, stop_time_details: &[StopTimeDetail]) -> Result<ServiceRoute> {
        let identify = self
            .to_identify(stop_time_details)
            .with_context(|| "service_routeのidentifyに失敗しました")?;
        match self.service_route_by_identify.get(&identify) {
            Some(c) => Ok(c.clone()),
            None => {
                let service_route_name = format!(
                    "{}({}～{})",
                    stop_time_details
                        .first()
                        .map_or("".to_string(), |x| x.clone().route_name()),
                    stop_time_details
                        .first()
                        .map_or("".to_string(), |x| x.stop_name.clone()),
                    stop_time_details
                        .last()
                        .map_or("".to_string(), |x| x.stop_name.clone())
                );
                let service_route = ServiceRoute {
                    service_route_id: self.service_route_id + 1,
                    service_route_name,
                    // TODO: TripのDirectionを使う
                    direction_id: Direction::Outbound,
                };
                println!("{}", service_route.clone().full_id());
                self.service_route_id += 1;
                self.service_route_by_identify
                    .insert(identify, service_route.clone());
                Ok(service_route)
            }
        }
    }

    pub fn all(&mut self) -> Vec<ServiceRoute> {
        self.service_route_by_identify
            .values()
            .cloned()
            .collect_vec()
    }
}
