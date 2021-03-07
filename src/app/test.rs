use anyhow::Result;

use crate::external;
use crate::external::gtfs::stops::Stop;

pub struct TestService {
    gtfs: Box<dyn external::gtfs::Gtfs>,
}

/// 開発動作確認用に好きな操作をさせるService
/// プロダクションでも使うコマンドは別途きちんと作成すること
impl TestService {
    pub fn new(gtfs: Box<dyn external::gtfs::Gtfs>) -> Self {
        Self { gtfs }
    }

    // pub fn fetch_agencies(&mut self) -> Result<Vec<Agency>> {
    //     self.gtfs.select_agencies()
    // }

    pub fn fetch_stops(&mut self) -> Result<Vec<Stop>> {
        self.gtfs.select_stops()
    }
}
