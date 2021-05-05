use anyhow::Result;
use clap::Clap;

use crate::cmd;

pub mod service_route_identity;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    /// データベースからservice_route_identityをコンバートする
    ServiceRouteIdentity(cmd::db::convert::service_route_identity::Opts),
}

pub fn run(opts: &Opts) -> Result<()> {
    match &opts.subcmd {
        SubCommand::ServiceRouteIdentity(op) => cmd::db::convert::service_route_identity::run(&op),
    }
}
