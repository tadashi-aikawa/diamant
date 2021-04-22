use anyhow::Result;
use clap::Clap;

use crate::cmd;

pub mod routes;
pub mod trips;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    /// GTFSファイルからrouteを取得する
    Routes(cmd::get::routes::Opts),
    /// GTFSファイルからtripを取得する
    Trips(cmd::get::trips::Opts),
}

pub fn run(opts: &Opts) -> Result<()> {
    match &opts.subcmd {
        SubCommand::Trips(op) => cmd::get::trips::run(&op),
        SubCommand::Routes(op) => cmd::get::routes::run(&op),
    }
}
