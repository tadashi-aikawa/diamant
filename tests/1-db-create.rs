use anyhow::Result;
use diamant::cmd;
use diamant::external::gtfs::agency::Agency;
use diamant::external::gtfs::extended::nodes::Node;
use diamant::external::gtfs::extended::service_routes::IdentifyStrategy;
use diamant::external::gtfs::{Lang, Timezone};
use std::path::PathBuf;

#[test]
fn no1_db_create() -> Result<()> {
    cmd::db::create::run(&cmd::db::create::Opts {
        gtfs_dir: PathBuf::from("tests/data"),
        database: PathBuf::from("gtfs.db"),
        legacy_translations: false,
        service_route_identify_strategy: IdentifyStrategy::StopNames,
        service_route_identify: None,
    })
}

#[test]
fn no2_agency_is_valid() -> Result<()> {
    let mut db = diamant::external::gtfsdb::GtfsDb::new("gtfs.db".as_ref())?;
    let agencies = db.select_all::<Agency>()?;
    assert_eq!(1, agencies.len());

    let agency = agencies.first().unwrap();
    assert_eq!(
        &Agency {
            agency_id: "33".to_string(),
            agency_name: "みみぞうバス".to_string(),
            agency_url: "https://minerva.mamansoft.net/".to_string(),
            agency_timezone: Timezone::AsiaTokyo,
            agency_lang: Some(Lang::Ja),
            agency_phone: None,
            agency_fare_url: None,
            agency_email: None
        },
        agency
    );
    Ok(())
}

#[test]
fn no3_nodes_is_valid() -> Result<()> {
    let mut db = diamant::external::gtfsdb::GtfsDb::new("gtfs.db".as_ref())?;
    let nodes = db.select_all::<Node>()?;
    assert_eq!(7, nodes.len());

    assert_eq!(
        vec![
            Node {
                node_id: 1,
                node_name: "日本橋".to_string(),
                node_ruby: "にほんばし".to_string(),
            },
            Node {
                node_id: 2,
                node_name: "茅場町".to_string(),
                node_ruby: "かやばちょう".to_string(),
            },
            Node {
                node_id: 3,
                node_name: "茅場町".to_string(),
                node_ruby: "かやばちょう".to_string(),
            },
            Node {
                node_id: 4,
                node_name: "清澄白河".to_string(),
                node_ruby: "きよすみしらかわ".to_string(),
            },
            Node {
                node_id: 5,
                node_name: "門前仲町".to_string(),
                node_ruby: "もんぜんなかちょう".to_string(),
            },
            Node {
                node_id: 6,
                node_name: "門前仲町".to_string(),
                node_ruby: "もんぜんなかちょう".to_string(),
            },
            Node {
                node_id: 7,
                node_name: "門前仲町".to_string(),
                node_ruby: "もんぜんなかちょう".to_string(),
            },
        ],
        nodes
    );
    Ok(())
}
