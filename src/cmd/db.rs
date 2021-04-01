use anyhow::Result;
use clap::Clap;

use crate::cmd;

pub mod create;
pub mod get;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    /// Create Database from GTFS files
    Create(cmd::db::create::Opts),
    /// Get records from GTFS DB
    Get(cmd::db::get::Opts),
}

pub fn run(opts: &Opts) -> Result<()> {
    match &opts.subcmd {
        SubCommand::Create(op) => cmd::db::create::run(op),
        SubCommand::Get(op) => cmd::db::get::run(op),
    }
}
