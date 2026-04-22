# Portwatch - Port & Process Monitor

Portwatch is a Rust CLI to scan open localhost ports, show related processes, and monitor changes in watch mode (plain output or TUI).

## Tools Used

- Rust (edition 2024)
- Tokio (async runtime)
- Clap (CLI parser)
- Ratatui + Crossterm (terminal UI)
- Serde + Serde JSON (JSON output)
- Colored (colored terminal output)
- Anyhow (error handling)

System tools used by scanner (macOS):

- `netstat`
- `lsof`

## Available CLI

### 1) Scan once

- Basic:
  - `cargo run -- scan`
- With minimum port:
  - `cargo run -- scan --min-port 3000`
- JSON output:
  - `cargo run -- scan --json`

### 2) Watch mode (auto refresh)

- Plain watch:
  - `cargo run -- watch`
- Plain watch with custom interval:
  - `cargo run -- watch --interval 5`
- TUI watch:
  - `cargo run -- watch --tui`
- TUI watch with custom interval:
  - `cargo run -- watch --tui --interval 10`

## TUI Controls

- `q` / `Esc`: quit
- `Up/Down`: navigate rows
- `r`: refresh now
