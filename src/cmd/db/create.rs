use std::path::PathBuf;

use anyhow::Result;
use clap::Clap;
use strum::VariantNames;

use crate::app::gtfs::GtfsService;
use crate::external;
use crate::external::gtfs::extended::service_routes;

#[derive(Clap, Debug)]
pub struct Opts {
    /// 読みこむGTFSが配置されたディレクトリのパス
    #[clap(parse(from_os_str))]
    gtfs_dir: PathBuf,
    /// 作成するデータベースファイルのパス
    #[clap(short, long, parse(from_os_str), default_value = "gtfs.db")]
    database: PathBuf,
    /// translationsの古い定義を使うかどうか
    #[clap(short, long)]
    legacy_translations: bool,
    /// service_routeの一意性戦略
    #[clap(
        short,
        long,
        possible_values(service_routes::IdentifyStrategy::VARIANTS)
    )]
    service_route_identify_strategy: service_routes::IdentifyStrategy,
}

pub fn run(op: &Opts) -> Result<()> {
    let gtfs_csv = external::gtfscsv::GtfsCsv::new(&op.gtfs_dir)?;
    let gtfs_db = external::gtfsdb::init(&op.database)?;

    let mut service = GtfsService::new(gtfs_csv, gtfs_db);

    service.drop_tables()?;
    service.create_tables()?;
    service.insert_tables(op.legacy_translations)?;
    service.insert_origin_tables(&op.service_route_identify_strategy)?;

    Ok(())
}
