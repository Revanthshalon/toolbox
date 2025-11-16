use std::{fmt::Display, net::IpAddr};

#[derive(Debug, Clone)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub enum Host {
    IpAddr(IpAddr),
    DomainAddr(String),
}

impl Display for Host {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IpAddr(ip_addr) => ip_addr.fmt(f),
            Self::DomainAddr(addr_str) => addr_str.fmt(f),
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum LogLevel {
    Trace,
    #[default]
    Debug,
    Info,
    Warn,
    Error,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Trace => tracing_core::Level::TRACE.fmt(f),
            Self::Debug => tracing_core::Level::DEBUG.fmt(f),
            Self::Info => tracing_core::Level::INFO.fmt(f),
            Self::Warn => tracing_core::Level::WARN.fmt(f),
            Self::Error => tracing_core::Level::ERROR.fmt(f),
        }
    }
}

impl LogLevel {
    pub fn get_log_level(&self) -> tracing_core::Level {
        match self {
            Self::Trace => tracing_core::Level::TRACE,
            Self::Debug => tracing_core::Level::DEBUG,
            Self::Info => tracing_core::Level::INFO,
            Self::Warn => tracing_core::Level::WARN,
            Self::Error => tracing_core::Level::ERROR,
        }
    }
}

#[derive(Debug, Default, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "serde", serde(rename_all = "lowercase"))]
pub enum LogFormat {
    Pretty,
    #[default]
    Json,
}

impl std::fmt::Display for LogFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Pretty => writeln!(f, "pretty"),
            Self::Json => writeln!(f, "json"),
        }
    }
}
