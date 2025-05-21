use bigdecimal::BigDecimal;
use chrono::{NaiveDate, NaiveDateTime};
use std::fmt::Display;

/// 通用 Option<T> 格式化为 String（ToString 实现类型）
pub fn fmt_opt<T: ToString>(v: &Option<T>) -> String {
    v.as_ref().map(ToString::to_string).unwrap_or_else(|| "null".to_string())
}

/// Option<T> 格式化为 String，支持自定义默认值
pub fn fmt_opt_or<T: ToString>(v: &Option<T>, default: &str) -> String {
    v.as_ref().map(ToString::to_string).unwrap_or_else(|| default.to_string())
}

/// Option<NaiveDate> 格式化为 YYYY-MM-DD
pub fn fmt_naive_date(v: &Option<NaiveDate>) -> String {
    v.map(|d| d.format("%Y-%m-%d").to_string()).unwrap_or_else(|| "null".to_string())
}

/// Option<NaiveDateTime> 格式化为 YYYY-MM-DD HH:MM:SS
pub fn fmt_naive_datetime(v: &Option<NaiveDateTime>) -> String {
    v.map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string()).unwrap_or_else(|| "null".to_string())
}

/// Option<BigDecimal> 转换为字符串（兼容日志格式）
pub fn fmt_bigdecimal(v: &Option<BigDecimal>) -> String {
    v.as_ref().map(ToString::to_string).unwrap_or_else(|| "null".to_string())
}

/// 简洁日志输出宏（支持多字段）
#[macro_export]
macro_rules! log_fields {
    ($level:ident, $( $label:expr => $value:expr ),+ $(,)?) => {
        log::$level!(
            "{}",
            vec![$(format!("{}: {}", $label, $value)),+].join(", ")
        );
    };
}
