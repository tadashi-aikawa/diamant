use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;

use crate::app::route::RouteService;
use crate::io::Format;
use crate::{external, io};
use strum::VariantNames;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(short, long, parse(from_os_str), default_value = "hibou.db")]
    database: PathBuf,
    #[clap(short, long)]
    route_id: Option<String>,
    #[clap(short, long, default_value = "csv", possible_values(Format::VARIANTS))]
    format: Format,
}

pub fn run(op: &Opts) -> Result<()> {
    let gtfs = external::gtfsdb::init(&op.database)?;
    let routes = RouteService::new(gtfs).fetch(&op.route_id)?;
    io::write(&routes, &op.format)?;
    Ok(())
}
