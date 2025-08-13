use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Connection {
    pub name: String,
    pub user: String,
    pub host: String,
    pub port: Option<u16>,
    /// If None, defaults to "xterm-256color" on connect
    pub term: Option<String>,
}

impl Connection {
    pub fn label(&self) -> String {
        let port = self.port.map(|p| format!(":{}", p)).unwrap_or_default();
        format!("{}  {}@{}{}", self.name, self.user, self.host, port)
    }
}