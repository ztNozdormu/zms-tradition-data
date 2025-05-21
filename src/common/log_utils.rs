use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use std::fmt::Display;
use tracing::info;

/// 通用 Option<T> 格式化为 String（ToString 实现类型）
pub fn fmt_opt<T: ToString>(v: &Option<T>) -> String {
    v.as_ref()
        .map(ToString::to_string)
        .unwrap_or_else(|| "null".to_string())
}

/// Option<T> 格式化为 String，支持自定义默认值
pub fn fmt_opt_or<T: ToString>(v: &Option<T>, default: &str) -> String {
    v.as_ref()
        .map(ToString::to_string)
        .unwrap_or_else(|| default.to_string())
}

/// Option<NaiveDate> 格式化为 YYYY-MM-DD
pub fn fmt_naive_date(v: &Option<NaiveDate>) -> String {
    v.map(|d| d.format("%Y-%m-%d").to_string())
        .unwrap_or_else(|| "null".to_string())
}

/// Option<NaiveDateTime> 格式化为 YYYY-MM-DD HH:MM:SS
pub fn fmt_naive_datetime(v: &Option<NaiveDateTime>) -> String {
    v.map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "null".to_string())
}

/// Option<BigDecimal> 转换为字符串（兼容日志格式）
pub fn fmt_bigdecimal(v: &Option<BigDecimal>) -> String {
    v.as_ref()
        .map(ToString::to_string)
        .unwrap_or_else(|| "null".to_string())
}

/// 简洁结构化日志输出宏，使用 tracing 实现
/// 基于 tracing 的多字段日志输出宏（拼接字符串形式）
#[macro_export]
macro_rules! trace_fields {
    ($level:ident, $( $label:expr => $value:expr ),+ $(,)?) => {
        tracing::$level!(
            "{}",
            vec![$(format!("{}: {}", $label, $value)),+].join(", ")
        );
    };
}
/// 使用方式
/// trace_fields!(info, "symbol" => symbol, "rank" => fmt_opt(&rank));
/// kv
#[macro_export]
macro_rules! trace_kv {
    ($level:ident, $( $key:ident = $val:expr ),+ $(,)?) => {
        tracing::$level!( $( $key = %$val ),+ );
    };
}

// 使用方式
// trace_kv!(info, symbol = symbol, rank = fmt_opt(&rank));

// tracing，它支持原生的结构化 key-value 日志
// info!(symbol = %symbol, rank = %fmt_opt(&rank), "Coin info");
