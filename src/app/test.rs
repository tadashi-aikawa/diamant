use anyhow::Result;

use crate::external;
use crate::external::gtfs::calendar_dates::CalendarDate;

pub struct TestService {
    gtfs: Box<dyn external::gtfs::Gtfs>,
}

/// 開発動作確認用に好きな操作をさせるService
/// プロダクションでも使うコマンドは別途きちんと作成すること
impl TestService {
    pub fn new(gtfs: Box<dyn external::gtfs::Gtfs>) -> Self {
        Self { gtfs }
    }

    pub fn fetch(&mut self) -> Result<Vec<CalendarDate>> {
        self.gtfs.select_calendar_dates()
    }
}
