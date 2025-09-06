use chrono::{DateTime, Utc};
use sea_orm::prelude::DateTimeUtc;
use serde::{Deserialize, Serialize, Serializer};

/// 将 DateTimeUtc 格式化为 yyyy-MM-dd HH:mm:SS 格式的字符串
pub fn format_datetime_utc(datetime: &DateTimeUtc) -> String {
    // 将 DateTimeUtc 转换为 chrono::DateTime<Utc>
    let chrono_datetime: DateTime<Utc> =
        DateTime::from_timestamp(datetime.timestamp(), datetime.timestamp_subsec_nanos())
            .unwrap_or_default();

    // 格式化为 yyyy-MM-dd HH:mm:SS
    chrono_datetime.format("%Y-%m-%d %H:%M:%S").to_string()
}

/// 自定义序列化器，用于将 DateTimeUtc 序列化为 yyyy-MM-dd HH:mm:SS 格式的字符串
pub fn serialize_datetime_utc<S>(datetime: &DateTimeUtc, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let formatted = format_datetime_utc(datetime);
    formatted.serialize(serializer)
}

/// 自定义反序列化器，用于将 yyyy-MM-dd HH:mm:SS 格式的字符串反序列化为 DateTime<Utc>
pub fn deserialize_datetime_utc<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let s: String = String::deserialize(deserializer)?;

    // 解析 yyyy-MM-dd HH:mm:SS 格式的字符串
    let parsed_datetime = chrono::DateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S")
        .map_err(serde::de::Error::custom)?
        .with_timezone(&Utc);

    // 转换为 UTC 时间戳
    let timestamp = parsed_datetime.timestamp();
    let nanos = parsed_datetime.timestamp_subsec_nanos();

    // 创建 DateTimeUtc
    Ok(DateTimeUtc::from_timestamp(timestamp, nanos).unwrap_or_default())
}
