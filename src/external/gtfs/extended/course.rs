use crate::external::gtfs::extended::trip_with_sequence_meta::TripWithSequenceMeta;
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

    fn to_identify(&self, trip_with_stops: &[TripWithSequenceMeta]) -> Result<String> {
        let first_stop = trip_with_stops.first().unwrap();
        match self.identify_strategy {
            IdentifyStrategy::StopIds => {
                Ok(trip_with_stops.iter().map(|x| x.stop_id.clone()).join(","))
            }
            IdentifyStrategy::StopNames => Ok(trip_with_stops
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

    /// trip_with_stopsは 1つのtripに対し、sequence昇順
    pub fn generate(&mut self, trip_with_stops: &[TripWithSequenceMeta]) -> Result<Course> {
        let identify = self
            .to_identify(trip_with_stops)
            .with_context(|| "courseのidentifyに失敗しました")?;
        match self.course_by_identify.get(&identify) {
            Some(c) => Ok(c.clone()),
            None => {
                let course_name = format!(
                    "{}({}～{})",
                    trip_with_stops
                        .first()
                        .map_or("".into(), |x| x.clone().route_name()),
                    trip_with_stops
                        .first()
                        .map_or("".into(), |x| x.stop_name.clone()),
                    trip_with_stops
                        .last()
                        .map_or("".into(), |x| x.stop_name.clone())
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
