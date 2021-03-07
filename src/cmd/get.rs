pub mod routes;
pub mod trips;

use crate::cmd;
use clap::Clap;

#[derive(Clap, Debug)]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,
}

#[derive(Clap, Debug)]
pub enum SubCommand {
    /// routes
    Routes(cmd::get::routes::Opts),
    /// trips
    Trips(cmd::get::trips::Opts),
}
