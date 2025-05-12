use crate::model::cex::kline::{ArchiveProgress, MarketKline};

pub mod kline;

impl_table_record!(MarketKline, MarketKline, "market_klines");
impl_table_record!(ArchiveProgress, ArchiveProgress, "archive_progress");
