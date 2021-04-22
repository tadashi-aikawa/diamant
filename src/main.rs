#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

use anyhow::Result;
use clap::Clap;
use env_logger::Env;

mod api;
mod app;
mod cmd;
mod external;
mod io;
mod serde_chrono_custom;

#[derive(Clap, Debug)]
#[clap(version = clap::crate_version!(), author = "tadashi-aikawa")]
struct Opts {
    /// The path of the config file to load
    // #[clap(short, long, default_value = ".diamant.yaml")]
    // config: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
    // #[clap(short, long, parse(from_occurrences), global = true)]
    // verbose: i32,
}

#[derive(Clap, Debug)]
enum SubCommand {
    /// データベースに関するコマンド群
    Db(cmd::db::Opts),
    /// GTFSファイルからデータを取得するコマンド群
    Get(cmd::get::Opts),
    /// APIサーバーとして立ち上げる(データベースと連携)
    Serve(cmd::serve::Opts),
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::Db(op) => cmd::db::run(&op)?,
        SubCommand::Get(op) => cmd::get::run(&op)?,
        SubCommand::Serve(op) => cmd::serve::run(&op),
    }

    Ok(())
}
