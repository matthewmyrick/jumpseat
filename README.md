# Jumpseat

A minimal SSH connection manager with a Terminal User Interface (TUI) built in Rust.

## Features

- **Fuzzy Search**: Quickly find connections using fuzzy matching
- **TUI Interface**: Clean, keyboard-driven interface using ratatui
- **Connection Management**: Add, delete, and organize SSH connections
- **Custom Terminal Types**: Configure terminal type per connection
- **Persistent Storage**: Connections saved in JSON format

## Installation

```bash
cargo build --release
```

## Usage

Run the application:

```bash
cargo run
```

### Key Bindings

- `/` - Start fuzzy search
- `a` - Add new connection
- `d` - Delete selected connection
- `Enter` - Connect to selected host
- `h` - Show help
- `q` - Quit
- `↑/↓` or `j/k` - Navigate connections

### Adding Connections

When adding a connection, use the format:
```
<name> <user>@<host>[:port] [term]
```

Example:
```
home matthew@192.168.1.10:22 xterm-256color
```

## Configuration

Connections are stored in a JSON file at:
- macOS/Linux: `~/.config/rssh/connections.json`
