use log::{debug, trace};
use rusqlite::NO_PARAMS;
use rusqlite::{params, Connection};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct StopTime {
    trip_id: String,
    arrival_time: String,
    departure_time: String,
    stop_id: String,
    stop_sequence: i32,
    stop_headsign: Option<String>,
    pickup_type: Option<i32>,
    drop_off_type: Option<i32>,
    shape_dist_traveled: Option<i32>,
    timepoint: Option<i32>,
}

pub fn create(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS stop_times (
            trip_id text,
            arrival_time datetime not null,
            departure_time datetime not null,
            stop_id text not null,
            stop_sequence int,
            stop_headsign text,
            pickup_type int,
            drop_off_type int,
            shape_dist_traveled int,
            timepoint int,
            PRIMARY KEY(trip_id, stop_sequence)
        )",
        NO_PARAMS,
    )?;
    debug!("Create table `stop_times`");
    Ok(())
}

pub fn drop(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("DROP TABLE IF EXISTS stop_times", NO_PARAMS)?;
    debug!("Drop table `stop_times`");
    Ok(())
}

pub fn insert(conn: &mut Connection, stop_times: &[StopTime]) -> rusqlite::Result<()> {
    let tx = conn.transaction()?;

    debug!("Insert {} records to stop_times", stop_times.len());
    for stop_times in stop_times {
        tx.execute(
            "INSERT INTO stop_times (
            trip_id,
            arrival_time,
            departure_time,
            stop_id,
            stop_sequence,
            stop_headsign,
            pickup_type,
            drop_off_type,
            shape_dist_traveled,
            timepoint
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                stop_times.trip_id,
                stop_times.arrival_time,
                stop_times.departure_time,
                stop_times.stop_id,
                stop_times.stop_sequence,
                stop_times.stop_headsign,
                stop_times.pickup_type,
                stop_times.drop_off_type,
                stop_times.shape_dist_traveled,
                stop_times.timepoint,
            ],
        )?;
    }

    tx.commit()?;

    trace!("Insert {:?}", stop_times);
    Ok(())
}
