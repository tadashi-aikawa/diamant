use anyhow::Result;
use clap::Clap;

use crate::cmd;

pub mod routes;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    /// データベースからrouteを取得する
    Routes(cmd::db::get::routes::Opts),
}

pub fn run(opts: &Opts) -> Result<()> {
    match &opts.subcmd {
        SubCommand::Routes(op) => cmd::db::get::routes::run(&op),
    }
}
