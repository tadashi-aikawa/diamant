use anyhow::Result;
use clap::Clap;

use crate::cmd;

pub mod routes;
pub mod trip_with_sequence_meta;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    /// データベースからrouteを取得する
    Routes(cmd::db::get::routes::Opts),
    /// データベースからtripに対して停車するstopとその付随情報を出力する
    TripWithSequenceMeta(cmd::db::get::trip_with_sequence_meta::Opts),
}

pub fn run(opts: &Opts) -> Result<()> {
    match &opts.subcmd {
        SubCommand::Routes(op) => cmd::db::get::routes::run(&op),
        SubCommand::TripWithSequenceMeta(op) => cmd::db::get::trip_with_sequence_meta::run(&op),
    }
}
