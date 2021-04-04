use crate::external::gtfs::extended::trip_with_stops::TripWithStop;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// コースID (ex: 1)
pub type CourseId = i32;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct Course {
    /// コースID
    pub course_id: CourseId,
    /// コース名
    pub course_name: String,
}

/// 同一性
type Identify = String;

pub struct CourseGenerator {
    current_id: CourseId,
    pub course_by_identify: HashMap<Identify, Course>,
}

impl CourseGenerator {
    pub fn new() -> Self {
        CourseGenerator {
            current_id: 0,
            course_by_identify: HashMap::new(),
        }
    }

    /// trip_with_stopsは 1つのtripに対し、sequence昇順
    pub fn generate(&mut self, trip_with_stops: &[TripWithStop]) -> Course {
        let identify = trip_with_stops
            .iter()
            .map(|x| x.stop_name.clone())
            .join(",");
        match self.course_by_identify.get(&identify) {
            Some(c) => c.clone(),
            None => {
                let course = Course {
                    course_id: self.current_id + 1,
                    course_name: "hoge".into(),
                };
                self.current_id += 1;
                self.course_by_identify.insert(identify, course.clone());
                course
            }
        }
    }
}
