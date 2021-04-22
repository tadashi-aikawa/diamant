use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;
use strum::VariantNames;

use crate::app::trip::{TripService, TripServiceCsv};
use crate::io::Format;
use crate::{external, io};

#[derive(Clap, Debug)]
pub struct Opts {
    /// 読みこむGTFSが配置されたディレクトリのパス
    #[clap(parse(from_os_str))]
    gtfs_dir: PathBuf,
    /// 出力フォーマット
    #[clap(short, long, default_value = "csv", possible_values(Format::VARIANTS))]
    format: Format,
}

pub fn run(op: &Opts) -> Result<()> {
    let gtfs = external::gtfscsv::GtfsCsv::new(&op.gtfs_dir)?;
    let trips = TripServiceCsv::new(gtfs).fetch()?;
    io::write(&trips, &op.format)?;
    Ok(())
}
