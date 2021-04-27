use crate::external::gtfs::extended::stop_time_details::StopTimeDetail;
use crate::external::gtfsdb::Table;
use anyhow::{anyhow, Context, Result};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum_macros::{EnumString, EnumVariantNames};

/// コースID (ex: 1)
pub type CourseId = i32;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct Course {
    /// コースID
    pub course_id: CourseId,
    /// コース名
    pub course_name: String,
}

impl Table for Course {
    fn table_name() -> &'static str {
        "courses"
    }

    fn column_names() -> &'static [&'static str] {
        &["course_id", "course_name"]
    }

    fn create_sql() -> &'static str {
        "
        course_id int,
        course_name text,
        PRIMARY KEY(course_id, course_name)
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

pub struct CourseGenerator {
    current_id: CourseId,
    pub course_by_identify: HashMap<Identify, Course>,
    identify_strategy: IdentifyStrategy,
}

impl CourseGenerator {
    pub fn new(strategy: &IdentifyStrategy) -> Self {
        CourseGenerator {
            current_id: 0,
            course_by_identify: HashMap::new(),
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
    pub fn generate(&mut self, stop_time_details: &[StopTimeDetail]) -> Result<Course> {
        let identify = self
            .to_identify(stop_time_details)
            .with_context(|| "courseのidentifyに失敗しました")?;
        match self.course_by_identify.get(&identify) {
            Some(c) => Ok(c.clone()),
            None => {
                let course_name = format!(
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
                let course = Course {
                    course_id: self.current_id + 1,
                    course_name,
                };
                self.current_id += 1;
                self.course_by_identify.insert(identify, course.clone());
                Ok(course)
            }
        }
    }

    pub fn all(&mut self) -> Vec<Course> {
        self.course_by_identify.values().cloned().collect_vec()
    }
}
