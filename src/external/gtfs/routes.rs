use log::{debug, trace};
use rusqlite::NO_PARAMS;
use rusqlite::{params, Connection};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Route {
    route_id: String,
    agency_id: String,
    route_short_name: Option<String>,
    route_long_name: Option<String>,
    route_desc: Option<String>,
    route_type: String,
    route_url: Option<String>,
    route_color: Option<String>,
    route_text_color: Option<String>,
    jp_parent_route_id: Option<String>,
}

pub fn create(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS routes (
            route_id text primary key,
            agency_id text not null,
            route_short_name text,
            route_long_name text,
            route_desc text,
            route_type text not null,
            route_url text,
            route_color text,
            route_text_color text,
            jp_parent_route_id text
        )",
        NO_PARAMS,
    )?;
    debug!("Create table `routes`");
    Ok(())
}

pub fn drop(conn: &Connection) -> rusqlite::Result<()> {
    conn.execute("DROP TABLE IF EXISTS routes", NO_PARAMS)?;
    debug!("Drop table `routes`");
    Ok(())
}

pub fn insert(conn: &mut Connection, routes: &[Route]) -> rusqlite::Result<()> {
    let tx = conn.transaction()?;

    debug!("Insert {} records to routes", routes.len());
    for route in routes {
        trace!("Insert {:?}", route);
        tx.execute(
            "INSERT INTO routes (
            route_id,
            agency_id,
            route_short_name,
            route_long_name,
            route_desc,
            route_type,
            route_url,
            route_color,
            route_text_color,
            jp_parent_route_id
        ) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
            params![
                route.route_id,
                route.agency_id,
                route.route_short_name,
                route.route_long_name,
                route.route_desc,
                route.route_type,
                route.route_url,
                route.route_color,
                route.route_text_color,
                route.jp_parent_route_id,
            ],
        )?;
    }

    tx.commit()?;

    Ok(())
}
