use chrono::Duration;
use serde::{Deserialize, Serialize};

pub mod cex;
pub mod constant;
pub mod dex;

pub static DEFAULT_TIMEFRAMES: &[TimeFrame] = &[
    TimeFrame::M1,
    TimeFrame::M5,
    TimeFrame::M15,
    TimeFrame::M30,
    TimeFrame::H1,
    TimeFrame::H2,
    TimeFrame::H4,
    TimeFrame::H8,
    TimeFrame::H12,
    TimeFrame::D1,
];


#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeFrame {
    M1,
    M3,
    M5,
    M15,
    M30,
    H1,
    H2,
    H4,
    H6,
    H8,
    H12,
    D1,
    D3,
    W1,
    M1L,
}

impl TimeFrame {
    // 获取对应的时间毫秒数
    pub fn to_millis(&self) -> i64 {
        match self {
            TimeFrame::M1 => Duration::minutes(1).num_milliseconds(),
            TimeFrame::M3 => Duration::minutes(3).num_milliseconds(),
            TimeFrame::M5 => Duration::minutes(5).num_milliseconds(),
            TimeFrame::M15 => Duration::minutes(15).num_milliseconds(),
            TimeFrame::M30 => Duration::minutes(30).num_milliseconds(),
            TimeFrame::H1 => Duration::hours(1).num_milliseconds(),
            TimeFrame::H2 => Duration::hours(2).num_milliseconds(),
            TimeFrame::H4 => Duration::hours(4).num_milliseconds(),
            TimeFrame::H6 => Duration::hours(6).num_milliseconds(),
            TimeFrame::H8 => Duration::hours(8).num_milliseconds(),
            TimeFrame::H12 => Duration::hours(12).num_milliseconds(),
            TimeFrame::D1 => Duration::days(1).num_milliseconds(),
            TimeFrame::D3 => Duration::days(3).num_milliseconds(),
            TimeFrame::W1 => Duration::days(7).num_milliseconds(),
            TimeFrame::M1L => Duration::days(30).num_milliseconds(),
        }
    }

    // period
    pub fn to_period(&self) -> i64 {
        match self {
            TimeFrame::M1 => constant::M1,
            TimeFrame::M3 => constant::M3,
            TimeFrame::M5 => constant::M5,
            TimeFrame::M15 => constant::M15,
            TimeFrame::M30 => constant::M30,
            TimeFrame::H1 => constant::H1,
            TimeFrame::H2 => constant::H2,
            TimeFrame::H4 => constant::H4,
            TimeFrame::H6 => constant::H6,
            TimeFrame::H8 => constant::H8,
            TimeFrame::H12 => constant::H12,
            TimeFrame::D1 => constant::D1,
            TimeFrame::D3 => constant::D3,
            TimeFrame::W1 => constant::W1,
            TimeFrame::M1L => constant::M1L,
        }
    }

    // 获取字符串表示
    pub fn to_str(&self) -> &str {
        match self {
            TimeFrame::M1 => "1m",
            TimeFrame::M3 => "3m",
            TimeFrame::M5 => "5m",
            TimeFrame::M15 => "15m",
            TimeFrame::M30 => "30m",
            TimeFrame::H1 => "1h",
            TimeFrame::H2 => "2h",
            TimeFrame::H4 => "4h",
            TimeFrame::H6 => "6h",
            TimeFrame::H8 => "8h",
            TimeFrame::H12 => "12h",
            TimeFrame::D1 => "1d",
            TimeFrame::D3 => "3d",
            TimeFrame::W1 => "1w",
            TimeFrame::M1L => "1M",
        }
    }
}
