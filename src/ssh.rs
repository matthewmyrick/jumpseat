use anyhow::{Context, Result};
use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::{
    io::{self, Write},
    process::Command,
};

use crate::models::Connection;

pub fn run_ssh(conn: &Connection) -> Result<()> {
    // Leave TUI, run ssh, then come back
    disable_raw_mode()?;
    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;

    let mut cmd = Command::new("ssh");
    cmd.arg(format!("{}@{}", conn.user, conn.host));
    if let Some(p) = conn.port {
        cmd.args(["-p", &p.to_string()]);
    }
    cmd.env(
        "TERM",
        conn.term.as_deref().unwrap_or("xterm-256color"),
    );

    let status = cmd.status().context("failed to launch ssh")?;

    eprintln!(
        "\n[ jumpseat ] ssh exited with status: {} (press any key to return)",
        status
    );
    let _ = io::stdout().flush();

    // Wait for a keypress so users can read messages
    let _ = crossterm::event::read();

    // Re-enter TUI
    execute!(
        io::stdout(),
        EnterAlternateScreen,
        EnableMouseCapture
    )?;
    enable_raw_mode()?;
    Ok(())
}