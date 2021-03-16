use anyhow::Result;

use crate::external;
use crate::external::gtfs::shapes::Shape;

pub struct TestService<G>
where
    G: external::gtfs::Gtfs,
{
    gtfs: G,
}

/// 開発動作確認用に好きな操作をさせるService
/// プロダクションでも使うコマンドは別途きちんと作成すること
impl<G> TestService<G>
where
    G: external::gtfs::Gtfs,
{
    pub fn new(gtfs: G) -> Self {
        Self { gtfs }
    }

    pub fn fetch(&mut self) -> Result<Vec<Shape>> {
        self.gtfs.select_shapes()
    }
}
