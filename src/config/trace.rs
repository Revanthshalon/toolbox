use crate::config::common::{LogFormat, LogLevel};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub struct TraceConfig {
    log_level: Option<LogLevel>,
    log_format: Option<LogFormat>,
}

impl TraceConfig {
    pub fn log_level(&self) -> LogLevel {
        self.log_level.unwrap_or_default()
    }

    pub fn log_format(&self) -> LogFormat {
        self.log_format.unwrap_or_default()
    }
}
