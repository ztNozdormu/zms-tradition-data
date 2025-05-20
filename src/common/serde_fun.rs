use bigdecimal::BigDecimal;
use chrono::{DateTime, NaiveDateTime};
use serde::{self, Deserialize, Deserializer, Serialize};
use serde_json::Value;
use std::str::FromStr;

pub fn deserialize<'de, D>(deserializer: D) -> Result<BigDecimal, D::Error>
where
    D: Deserializer<'de>,
{
    let s: String = Deserialize::deserialize(deserializer)?;
    BigDecimal::from_str(&s).map_err(serde::de::Error::custom)
}

pub fn deserialize_option_string2bigdcimal<'de, D>(
    deserializer: D,
) -> Result<Option<BigDecimal>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum BigDecimalRaw {
        String(String),
        Float(f64),
        Null,
    }

    match BigDecimalRaw::deserialize(deserializer)? {
        BigDecimalRaw::String(s) => {
            if s.to_lowercase() == "null" {
                Ok(None)
            } else {
                BigDecimal::from_str(&s)
                    .map(Some)
                    .map_err(serde::de::Error::custom)
            }
        }
        BigDecimalRaw::Float(f) => {
            let s = f.to_string(); // 间接转换以保证精度
            BigDecimal::from_str(&s)
                .map(Some)
                .map_err(serde::de::Error::custom)
        }
        BigDecimalRaw::Null => Ok(None),
    }
}

const FORMAT: &str = "%Y-%m-%d %H:%M:%S";

pub fn deserialize_datetime_option<'de, D>(
    deserializer: D,
) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: Deserializer<'de>,
{
    let opt = Option::<String>::deserialize(deserializer)?;
    if let Some(s) = opt {
        // 尝试以时区格式解析（ISO 8601）
        if let Ok(dt) = DateTime::parse_from_rfc3339(&s) {
            return Ok(Some(dt.naive_utc()));
        }

        // 支持 ISO8601 手动格式 +00:00 / +08:00 等
        let offset_formats = &[
            "%Y-%m-%dT%H:%M:%S%.f%:z", // 带毫秒和时区
            "%Y-%m-%dT%H:%M:%S%:z",    // 无毫秒，带时区
        ];
        for fmt in offset_formats {
            if let Ok(dt) = DateTime::parse_from_str(&s, fmt) {
                return Ok(Some(dt.naive_utc()));
            }
        }

        // 无时区格式：直接解析为 NaiveDateTime
        let naive_formats = &[
            "%Y-%m-%dT%H:%M:%S%.f", // 带毫秒
            "%Y-%m-%dT%H:%M:%S",    // 无毫秒
            "%Y-%m-%d %H:%M:%S%.f", // 空格分隔，带毫秒
            "%Y-%m-%d %H:%M:%S",    // 空格分隔，无毫秒
        ];
        for fmt in naive_formats {
            if let Ok(dt) = NaiveDateTime::parse_from_str(&s, fmt) {
                return Ok(Some(dt));
            }
        }

        Err(serde::de::Error::custom(format!(
            "Unrecognized datetime format: {}",
            s
        )))
    } else {
        Ok(None)
    }
}

/// 通用函数：将 Option<Vec<T>> 转换为 JSON Value，其中 T: Into<serde_json::Value>
pub fn option_vec_to_value<T>(input: Option<Vec<T>>) -> Value
where
    T: Into<Value>,
{
    match input {
        Some(vec) => Value::Array(vec.into_iter().map(Into::into).collect()),
        None => Value::Null,
    }
}

/// 将可序列化的值转换为 Option<serde_json::Value>
/// 如果序列化失败则返回 None
pub fn option_map_to_value<T: Serialize>(val: T) -> Option<Value> {
    serde_json::to_value(val).ok()
}
