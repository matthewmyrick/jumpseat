mod app;
mod config;
mod models;
mod ssh;
mod ui;

use std::{io, time::Duration};

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};

use app::{AppState, Mode};
use config::{add_from_line, load_connections, save_connections};
use ssh::run_ssh;
use ui::draw_ui;

fn handle_input(app: &mut AppState, event: Event) -> Result<bool> {
    match event {
        Event::Key(KeyEvent { code, modifiers, .. }) => {
            if app.pending_delete {
                match code {
                    KeyCode::Char('y') | KeyCode::Char('Y') => {
                        if let Some(i) = app.filtered_indices.get(app.selected).cloned() {
                            app.connections.remove(i);
                            let _ = save_connections(&app.connections);
                            app.status = "Deleted.".into();
                            app.apply_filter();
                        }
                        app.pending_delete = false;
                    }
                    _ => {
                        app.pending_delete = false;
                    }
                }
                return Ok(false);
            }

            match app.mode {
                Mode::Add => match code {
                    KeyCode::Esc => {
                        app.mode = Mode::Normal;
                        app.add_buffer.clear();
                        app.status.clear();
                    }
                    KeyCode::Enter => {
                        match add_from_line(&app.add_buffer) {
                            Ok(conn) => {
                                app.connections.push(conn);
                                let _ = save_connections(&app.connections);
                                app.add_buffer.clear();
                                app.mode = Mode::Normal;
                                app.status = "Saved.".into();
                                app.apply_filter();
                            }
                            Err(e) => {
                                app.status = format!("Error: {}", e);
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        app.add_buffer.pop();
                    }
                    KeyCode::Char(c) => {
                        app.add_buffer.push(c);
                    }
                    _ => {}
                },
                Mode::Search => match code {
                    KeyCode::Esc => {
                        app.mode = Mode::Normal;
                    }
                    KeyCode::Backspace => {
                        app.search.pop();
                        app.apply_filter();
                    }
                    KeyCode::Char(c) => {
                        // Allow Ctrl+u to clear
                        if modifiers.contains(KeyModifiers::CONTROL) && c == 'u' {
                            app.search.clear();
                        } else {
                            app.search.push(c);
                        }
                        app.apply_filter();
                    }
                    KeyCode::Enter => {
                        app.mode = Mode::Normal;
                    }
                    _ => {}
                },
                Mode::Normal => match code {
                    KeyCode::Char('q') => return Ok(true),
                    KeyCode::Char('h') => {
                        app.show_help = !app.show_help;
                    }
                    KeyCode::Char('/') => {
                        app.mode = Mode::Search;
                        app.status.clear();
                    }
                    KeyCode::Char('a') => {
                        app.mode = Mode::Add;
                        app.add_buffer.clear();
                        app.status.clear();
                    }
                    KeyCode::Char('d') => {
                        app.pending_delete = true;
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.selected > 0 {
                            app.selected -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.selected + 1 < app.filtered_indices.len() {
                            app.selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        if let Some(i) = app.filtered_indices.get(app.selected).cloned() {
                            let conn = app.connections[i].clone();
                            if let Err(e) = run_ssh(&conn) {
                                app.status = format!("SSH error: {}", e);
                            } else {
                                app.status = format!("Returned from {}", conn.name);
                            }
                        }
                    }
                    _ => {}
                },
            }
        }
        Event::Resize(_, _) => {}
        _ => {}
    }
    Ok(false)
}

fn main() -> Result<()> {
    // Load
    let mut app = AppState::default();
    app.connections = load_connections()?;
    app.apply_filter();

    // TUI init
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // Event loop
    loop {
        // UI
        terminal.draw(|f| draw_ui(f, &mut app))?;

        // Input
        if crossterm::event::poll(Duration::from_millis(200))? {
            if handle_input(&mut app, event::read()?)? {
                break;
            }
        }
    }

    // Cleanup
    disable_raw_mode()?;
    execute!(
        io::stdout(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    Ok(())
}