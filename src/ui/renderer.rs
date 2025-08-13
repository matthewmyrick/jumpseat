use ratatui::{
    layout::{Constraint, Direction, Layout},
    prelude::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

use crate::app::{AppState, Mode};
use super::widgets::{centered_rect, tui_list_state};

pub fn draw_ui(f: &mut Frame, app: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(2),
        ])
        .split(f.area());

    draw_search_bar(f, app, chunks[0]);
    draw_connections_list(f, app, chunks[1]);
    draw_footer(f, app, chunks[2]);
    
    // Draw dialogs on top
    if app.mode == Mode::Add || app.pending_delete || app.show_help {
        let area = centered_rect(80, 40, f.area());
        f.render_widget(Clear, area);
        
        if app.mode == Mode::Add {
            draw_add_dialog(f, app, area);
        } else if app.pending_delete {
            draw_delete_dialog(f, app, area);
        } else if app.show_help {
            draw_help_dialog(f, area);
        }
    }
}

fn draw_search_bar(f: &mut Frame, app: &AppState, area: Rect) {
    let search_title = match app.mode {
        Mode::Search => " 🔍 Search (ESC to cancel) ",
        _ => " Search (/ to start) ",
    };
    
    let search_block = Block::default()
        .title(search_title)
        .borders(Borders::ALL)
        .border_type(if app.mode == Mode::Search {
            BorderType::Thick
        } else {
            BorderType::Plain
        })
        .border_style(if app.mode == Mode::Search {
            Style::default().fg(Color::Cyan)
        } else {
            Style::default().fg(Color::DarkGray)
        })
        .style(if app.mode == Mode::Search {
            Style::default().bg(Color::Rgb(10, 10, 20))
        } else {
            Style::default()
        });
    
    let search = Paragraph::new(app.search.as_str())
        .style(if app.mode == Mode::Search {
            Style::default().fg(Color::White)
        } else {
            Style::default().fg(Color::Gray)
        })
        .block(search_block);
    f.render_widget(search, area);
}

fn draw_connections_list(f: &mut Frame, app: &mut AppState, area: Rect) {
    let items: Vec<ListItem> = app
        .filtered_indices
        .iter()
        .enumerate()
        .map(|(_idx, &i)| {
            let conn = &app.connections[i];
            let port = conn.port.map(|p| format!(":{}", p)).unwrap_or_default();
            
            let mut spans = vec![
                Span::styled(
                    format!("{:<20}", conn.name),
                    Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
                ),
                Span::raw("  "),
                Span::styled(
                    conn.user.clone(),
                    Style::default().fg(Color::Green)
                ),
                Span::styled("@", Style::default().fg(Color::Gray)),
                Span::styled(
                    format!("{}{}", conn.host, port),
                    Style::default().fg(Color::Yellow)
                ),
            ];
            
            if conn.term.is_some() {
                spans.push(Span::raw("  "));
                spans.push(Span::styled(
                    format!("[{}]", conn.term.as_ref().unwrap()),
                    Style::default().fg(Color::DarkGray)
                ));
            }
            
            ListItem::new(Line::from(spans))
        })
        .collect();

    let list_block = Block::default()
        .title(" 🖥️  Connections ")
        .borders(Borders::ALL)
        .border_type(if app.mode == Mode::Normal {
            BorderType::Thick
        } else {
            BorderType::Plain
        })
        .border_style(if app.mode == Mode::Normal {
            Style::default().fg(Color::Green)
        } else {
            Style::default().fg(Color::DarkGray)
        });

    let list = List::new(items)
        .block(list_block)
        .highlight_style(
            Style::default()
                .bg(Color::Rgb(50, 50, 50))
                .add_modifier(Modifier::BOLD)
        )
        .highlight_symbol("▶ ");
    f.render_stateful_widget(list, area, &mut tui_list_state(app.selected));
}

fn draw_footer(f: &mut Frame, app: &AppState, area: Rect) {
    let connection_count = if app.filtered_indices.is_empty() {
        "0/0".to_string()
    } else {
        format!(
            "{}/{}",
            app.selected + 1,
            app.filtered_indices.len()
        )
    };
    
    let mut footer_spans = vec![
        Span::styled("↑↓/jk", Style::default().fg(Color::Cyan)),
        Span::raw(": move  "),
        Span::styled("Enter", Style::default().fg(Color::Green)),
        Span::raw(": connect  "),
        Span::styled("a", Style::default().fg(Color::Yellow)),
        Span::raw(": add  "),
        Span::styled("d", Style::default().fg(Color::Red)),
        Span::raw(": delete  "),
        Span::styled("/", Style::default().fg(Color::Magenta)),
        Span::raw(": search  "),
        Span::styled("h", Style::default().fg(Color::Blue)),
        Span::raw(": help  "),
        Span::styled("q", Style::default().fg(Color::Red)),
        Span::raw(": quit   "),
        Span::styled(
            format!("[{}]", connection_count),
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
        ),
    ];
    
    if !app.status.is_empty() {
        footer_spans.push(Span::raw("  │  "));
        footer_spans.push(Span::styled(
            &app.status,
            Style::default().fg(Color::Yellow).add_modifier(Modifier::ITALIC)
        ));
    }
    
    let footer = Paragraph::new(Line::from(footer_spans))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::DarkGray))
        );
    f.render_widget(footer, area);
}

fn draw_add_dialog(f: &mut Frame, app: &AppState, area: Rect) {
    let add = Paragraph::new(app.add_buffer.as_str())
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .title(" ➕ Add Connection ")
                .title_bottom(" Enter=save, ESC=cancel ")
                .borders(Borders::ALL)
                .border_type(BorderType::Thick)
                .border_style(Style::default().fg(Color::Yellow))
                .style(Style::default().bg(Color::Rgb(20, 20, 30)))
        );
    f.render_widget(add, area);
    
    // Show format hint below input
    let hint_area = Rect {
        x: area.x + 2,
        y: area.y + 3,
        width: area.width - 4,
        height: 1,
    };
    let hint = Paragraph::new("Format: <name> <user>@<host>[:port] [term]")
        .style(Style::default().fg(Color::DarkGray).add_modifier(Modifier::ITALIC));
    f.render_widget(hint, hint_area);
}

fn draw_delete_dialog(f: &mut Frame, app: &AppState, area: Rect) {
    let idx = app.filtered_indices.get(app.selected).cloned();
    let msg = if let Some(i) = idx {
        vec![
            Line::from(vec![
                Span::raw("Delete connection '"),
                Span::styled(
                    &app.connections[i].name,
                    Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                ),
                Span::raw("' ?"),
            ]),
            Line::from(""),
            Line::from(vec![
                Span::raw("Press "),
                Span::styled("y", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(" to confirm or "),
                Span::styled("any other key", Style::default().fg(Color::Yellow)),
                Span::raw(" to cancel"),
            ]),
        ]
    } else {
        vec![Line::from("Nothing selected. Press any key.")]
    };
    let dlg = Paragraph::new(msg).block(
        Block::default()
            .title(" ⚠️  Confirm Delete ")
            .borders(Borders::ALL)
            .border_type(BorderType::Double)
            .border_style(Style::default().fg(Color::Red))
            .style(Style::default().bg(Color::Rgb(30, 10, 10)))
    );
    f.render_widget(dlg, area);
}

fn draw_help_dialog(f: &mut Frame, area: Rect) {
    let help_text = vec![
        Line::from(vec![
            Span::styled("Jumpseat", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
            Span::raw(" — SSH Connection Manager"),
        ]),
        Line::from(""),
        Line::from(Span::styled("Navigation", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from(vec![
            Span::styled("  ↑/↓ or j/k  ", Style::default().fg(Color::Cyan)),
            Span::raw("Navigate connections"),
        ]),
        Line::from(vec![
            Span::styled("  Enter       ", Style::default().fg(Color::Green)),
            Span::raw("Connect to selected host"),
        ]),
        Line::from(""),
        Line::from(Span::styled("Actions", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from(vec![
            Span::styled("  /           ", Style::default().fg(Color::Magenta)),
            Span::raw("Search (fuzzy matching)"),
        ]),
        Line::from(vec![
            Span::styled("  a           ", Style::default().fg(Color::Yellow)),
            Span::raw("Add new connection"),
        ]),
        Line::from(vec![
            Span::styled("  d           ", Style::default().fg(Color::Red)),
            Span::raw("Delete selected connection"),
        ]),
        Line::from(vec![
            Span::styled("  h           ", Style::default().fg(Color::Blue)),
            Span::raw("Toggle this help"),
        ]),
        Line::from(vec![
            Span::styled("  q           ", Style::default().fg(Color::Red)),
            Span::raw("Quit application"),
        ]),
        Line::from(""),
        Line::from(Span::styled("Add Format", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled("  <name> <user>@<host>[:port] [term]", Style::default().fg(Color::Gray))),
        Line::from(""),
        Line::from(Span::styled("Example", Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))),
        Line::from(Span::styled("  myserver alice@192.168.1.10:22 xterm-256color", Style::default().fg(Color::Gray))),
    ];
    let dlg = Paragraph::new(help_text).block(
        Block::default()
            .title(" ❓ Help ")
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(Color::Blue))
            .style(Style::default().bg(Color::Rgb(10, 10, 30)))
    );
    f.render_widget(dlg, area);
}