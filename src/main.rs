use anyhow::Result;
use clap::Clap;
use env_logger::Env;

mod app;
mod cmd;
mod external;
mod io;
mod serde_chrono_custom;

#[derive(Clap, Debug)]
#[clap(version = "0.1", author = "tadashi-aikawa")]
struct Opts {
    /// The path of the config file to load
    #[clap(short, long, default_value = ".hibou.yaml")]
    config: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
    // #[clap(short, long, parse(from_occurrences), global = true)]
    // verbose: i32,
}

#[derive(Clap, Debug)]
enum SubCommand {
    /// GTFS-JPからデータベースを作成します
    MakeDb(cmd::make_db::Opts),
    /// データベースからデータを取得します
    Get(cmd::get::Opts),
    /// 開発用のテストコマンド
    Test(cmd::test::Opts),
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    let opts: Opts = Opts::parse();
    match opts.subcmd {
        SubCommand::MakeDb(op) => cmd::make_db::run(&op)?,
        SubCommand::Get(op) => cmd::get::run(&op)?,
        SubCommand::Test(op) => cmd::test::run(&op)?,
    }

    Ok(())
}
