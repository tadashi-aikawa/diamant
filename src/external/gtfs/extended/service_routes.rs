use std::collections::HashMap;

use anyhow::{Context, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use strum_macros::{EnumString, EnumVariantNames};

use crate::external::gtfs::extended::service_route_identity::ServiceRouteIdentity;
use crate::external::gtfs::extended::stop_time_details::StopTimeDetail;
use crate::external::gtfs::DirectionId;
use crate::external::gtfsdb::Table;

/// サービスルートID (ex: 1)
pub type ServiceRouteId = i32;
// /// サービスルートフルID (ex: 1^1)
// pub type ServiceRouteFullId = String;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct ServiceRoute {
    /// サービスルートID
    pub service_route_id: ServiceRouteId,
    /// サービスルート名
    pub service_route_name: String,
    /// 上下区分
    pub direction_id: DirectionId,
}

// impl ServiceRoute {
//     fn full_id(self) -> ServiceRouteFullId {
//         format!(
//             "{}^{}",
//             self.service_route_id,
//             serde_json::to_string(&self.direction_id).unwrap()
//         )
//     }
// }

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
type Identifier = String;

#[derive(Debug, Clone, EnumString, EnumVariantNames)]
#[strum(serialize_all = "snake_case")]
pub enum IdentifyStrategy {
    StopIds,
    StopNames,
}

pub struct ServiceRouteGenerator {
    service_route_id: ServiceRouteId,
    pub service_route_by_identify: HashMap<Identifier, ServiceRoute>,
    identify_strategy: IdentifyStrategy,
}

impl ServiceRouteGenerator {
    pub fn new(
        strategy: &IdentifyStrategy,
        identities_or: Option<&Vec<ServiceRouteIdentity>>,
    ) -> Self {
        match identities_or {
            Some(identities) => {
                let max_id = identities
                    .iter()
                    .map(|i| i.service_route_id)
                    .max()
                    .unwrap_or(0);

                // 先にインスタンスを作成
                let mut ins = ServiceRouteGenerator {
                    service_route_id: max_id,
                    service_route_by_identify: HashMap::new(),
                    identify_strategy: strategy.clone(),
                };

                for identity in identities {
                    let service_route = ServiceRoute {
                        service_route_id: identity.service_route_id,
                        service_route_name: identity.service_route_name.clone(),
                        direction_id: identity.service_route_direction_id.clone(),
                    };
                    ins.service_route_by_identify
                        .insert(ins.pick_identifier(&identity), service_route);
                }

                ins
            }
            None => ServiceRouteGenerator {
                service_route_id: 0,
                service_route_by_identify: HashMap::new(),
                identify_strategy: strategy.clone(),
            },
        }
    }

    /// Identityファイルからidentifierを求める
    fn pick_identifier(&self, identity: &ServiceRouteIdentity) -> Identifier {
        match self.identify_strategy {
            IdentifyStrategy::StopIds => identity.stop_ids.clone(),
            IdentifyStrategy::StopNames => identity.stop_names.clone(),
        }
    }

    /// stop_time_detailsからidentifierを求める
    fn to_identifier(&self, stop_time_details: &[StopTimeDetail]) -> Result<Identifier> {
        match self.identify_strategy {
            IdentifyStrategy::StopIds => Ok(stop_time_details.iter().map(|x| &x.stop_id).join(",")),
            IdentifyStrategy::StopNames => {
                Ok(stop_time_details.iter().map(|x| &x.stop_name).join(","))
            }
        }
    }

    /// stop_time_detailsは 1つのtripに対し、sequence昇順
    pub fn generate(&mut self, stop_time_details: &[StopTimeDetail]) -> Result<ServiceRoute> {
        let identify = self
            .to_identifier(stop_time_details)
            .with_context(|| "service_routeのidentifyに失敗しました")?;
        match self.service_route_by_identify.get(&identify) {
            Some(c) => Ok(c.clone()),
            None => {
                let first_detail = stop_time_details
                    .first()
                    .expect("stop_time_detailsが存在しません。予期せぬGTFSデータです。");
                let last_detail = stop_time_details
                    .last()
                    .expect("stop_time_detailsが存在しません。予期せぬGTFSデータです。");

                // service_route_nameは参考程度で表示に使う想定はしていない
                let service_route_name = format!(
                    "{}({}～{})",
                    first_detail.route_name(),
                    first_detail.stop_name,
                    last_detail.stop_name,
                );
                let service_route = ServiceRoute {
                    service_route_id: self.service_route_id + 1,
                    service_route_name,
                    direction_id: first_detail
                        .direction_id
                        .clone()
                        .unwrap_or(DirectionId::Outbound),
                };
                self.service_route_id += 1;
                self.service_route_by_identify
                    .insert(identify, service_route.clone());
                Ok(service_route)
            }
        }
    }

    pub fn all(&self) -> Vec<&ServiceRoute> {
        self.service_route_by_identify.values().collect_vec()
    }
}
