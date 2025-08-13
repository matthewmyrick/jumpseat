use anyhow::{bail, Context, Result};
use directories::ProjectDirs;
use std::{fs, path::PathBuf};

use crate::models::Connection;

pub fn config_path() -> Result<PathBuf> {
    let proj = ProjectDirs::from("dev", "minimal", "rssh")
        .ok_or_else(|| anyhow::anyhow!("cannot resolve config dir"))?;
    let dir = proj.config_dir().to_path_buf();
    fs::create_dir_all(&dir)?;
    Ok(dir.join("connections.json"))
}

pub fn load_connections() -> Result<Vec<Connection>> {
    let path = config_path()?;
    if !path.exists() {
        Ok(vec![])
    } else {
        let data = fs::read_to_string(path)?;
        let v: Vec<Connection> = serde_json::from_str(&data)?;
        Ok(v)
    }
}

pub fn save_connections(conns: &[Connection]) -> Result<()> {
    let path = config_path()?;
    let data = serde_json::to_string_pretty(conns)?;
    fs::write(path, data)?;
    Ok(())
}

pub fn add_from_line(line: &str) -> Result<Connection> {
    // Format:  "<name> <user>@<host>[:port] [term]"
    // Minimal & forgiving: split by whitespace
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 2 {
        bail!("Add format: <name> <user>@<host>[:port] [term]");
    }
    let name = parts[0].to_string();
    let uh = parts[1];

    let (user, host_port) = uh
        .split_once('@')
        .context("must include user@host")?;
    let (host, port) = if let Some((h, p)) = host_port.split_once(':') {
        let port: u16 = p.parse().context("invalid port")?;
        (h.to_string(), Some(port))
    } else {
        (host_port.to_string(), None)
    };

    let term = if parts.len() >= 3 {
        Some(parts[2].to_string())
    } else {
        None
    };

    Ok(Connection {
        name,
        user: user.to_string(),
        host,
        port,
        term,
    })
}