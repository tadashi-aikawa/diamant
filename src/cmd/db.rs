use anyhow::Result;
use clap::Clap;

use crate::cmd;

pub mod create;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    /// Create Database from GTFS files
    Create(cmd::db::create::Opts),
}

pub fn run(opts: &Opts) -> Result<()> {
    match &opts.subcmd {
        SubCommand::Create(op) => cmd::db::create::run(op),
    }
}
