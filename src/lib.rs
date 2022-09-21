use fluvio_smartmodule::{smartmodule, Result, Record};

#[derive(PartialEq, Eq, PartialOrd, Ord, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
enum LogLevel {
    Debug,
    Info,
    Warn,
    Error
}

#[derive(serde::Deserialize)]
struct StructuredLog {
    level: LogLevel,
    #[serde(rename = "message")]
    _message: String,
}

#[smartmodule(filter)]
pub fn filter(record: &Record) -> Result<bool> {
    let log: StructuredLog = serde_json::from_slice(record.value.as_ref())?;

    Ok(log.level > LogLevel::Debug)
}
