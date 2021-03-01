mod cmd;
mod external;

use anyhow::Result;
use clap::Clap;
use env_logger::Env;
use log::{debug, info};

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "tadashi-aikawa")]
struct Opts {
    /// The path of the config file to load
    #[clap(short, long, default_value = ".hibou.yaml")]
    config: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
    #[clap(short, long, parse(from_occurrences), global = true)]
    verbose: i32,
}

#[derive(Clap, Debug)]
enum SubCommand {
    /// Test command
    MakeDb(cmd::make_db::Opts),
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let opts: Opts = Opts::parse();
    debug!("Options: {:?}", opts);
    match opts.subcmd {
        SubCommand::MakeDb(op) => {
            info!("Execute test commands");
            cmd::make_db::run(&op)?
        }
    }

    Ok(())
}
