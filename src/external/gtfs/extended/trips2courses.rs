use serde::{Deserialize, Serialize};

pub use crate::external::gtfs::trips::{Trip, TripId};

use crate::external::gtfs::extended::course::CourseId;
use crate::external::gtfsdb::Table;

#[derive(Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Hash)]
pub struct Trip2Course {
    /// 便ID
    pub trip_id: TripId,
    /// コースID
    pub course_id: CourseId,
}

impl Table for Trip2Course {
    fn table_name() -> &'static str {
        "trips2courses"
    }

    fn column_names() -> &'static [&'static str] {
        &["trip_id", "course_id"]
    }

    fn create_sql() -> &'static str {
        "
        trip_id text,
        course_id int,
        PRIMARY KEY(trip_id, course_id)
        "
    }
}
