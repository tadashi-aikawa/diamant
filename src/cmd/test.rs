use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;

use crate::app::test::TestService;
use crate::io::Format;
use crate::{external, io};
use strum::VariantNames;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(short, long, parse(from_os_str), default_value = "hibou.db")]
    database: PathBuf,
    #[clap(short, long, default_value = "csv", possible_values(Format::VARIANTS))]
    format: Format,
}

/// 開発動作確認用に好きな操作をさせるCommand
/// プロダクションでも使うコマンドは別途きちんと作成すること
pub fn run(op: &Opts) -> Result<()> {
    let gtfs = external::gtfsdb::init(&op.database)?;
    let routes = TestService::new(gtfs).fetch()?;
    io::write(&routes, &op.format)?;
    Ok(())
}
