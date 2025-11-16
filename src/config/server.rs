use crate::config::common::Host;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Deserialize))]
pub struct ServerConfig {
    host: Host,
    port: u16,
}

impl ServerConfig {
    pub fn get_address(&self) -> String {
        format!("{0}:{1}", self.get_host(), self.get_port())
    }

    pub fn get_host(&self) -> String {
        self.host.to_string()
    }

    pub fn get_port(&self) -> String {
        self.port.to_string()
    }
}
