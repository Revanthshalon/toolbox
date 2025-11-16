use std::time::Duration;

use crate::config::common::Host;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct DbConfig {
    host: Host,
    port: u16,
    username: String,
    password: String,
    database_name: String,
    max_connections: Option<u32>,
    min_connections: Option<u32>,
    acquire_timeout: Option<u64>,
    idle_timeout: Option<u64>,
}

impl DbConfig {
    pub fn get_connection_url(&self) -> String {
        format!(
            "postgres://{0}:{1}@{2}:{3}/{4}",
            self.username, self.password, self.host, self.port, self.database_name
        )
    }

    pub fn get_max_connections(&self) -> u32 {
        self.max_connections.unwrap_or(50)
    }

    pub fn get_min_connections(&self) -> u32 {
        self.min_connections.unwrap_or(30)
    }

    pub fn get_acquire_timeout(&self) -> Duration {
        Duration::from_secs(self.acquire_timeout.unwrap_or(15))
    }

    pub fn get_idle_timeout(&self) -> Duration {
        Duration::from_secs(self.idle_timeout.unwrap_or(300))
    }
}
