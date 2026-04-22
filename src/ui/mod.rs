pub mod app;

use anyhow::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use ratatui::{Terminal, backend::CrosstermBackend};
use std::io;
use std::time::Duration;
use tokio::sync::mpsc;

use crate::watcher::{WatchCommand, WatchEvent};
use app::App;

pub async fn run_tui(
    mut rx: mpsc::Receiver<WatchEvent>,
    cmd_tx: mpsc::Sender<WatchCommand>,
    interval: u64,
) -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new(interval);
    let result = run_event_loop(&mut terminal, &mut app, &mut rx, cmd_tx).await;

    // Cleanup terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;

    result
}

async fn run_event_loop(
    terminal: &mut ratatui::Terminal<ratatui::backend::CrosstermBackend<std::io::Stdout>>,
    app: &mut App,
    rx: &mut mpsc::Receiver<WatchEvent>,
    cmd_tx: mpsc::Sender<WatchCommand>,
) -> Result<()> {
    loop {
        terminal.draw(|f| app.render(f))?;

        if event::poll(Duration::from_millis(50))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => break,
                    KeyCode::Down => {
                        let next = app
                            .table_state
                            .selected()
                            .map(|i| (i + 1).min(app.ports.len().saturating_sub(1)))
                            .unwrap_or(0);
                        app.table_state.select(Some(next));
                    }
                    KeyCode::Up => {
                        let prev = app
                            .table_state
                            .selected()
                            .map(|i| i.saturating_sub(1))
                            .unwrap_or(1);
                        app.table_state.select(Some(prev));
                    }
                    KeyCode::Char('r') => {
                        let _ = cmd_tx.try_send(WatchCommand::RefreshNow);
                    }
                    _ => {}
                }
            }
        }

        while let Ok(event) = rx.try_recv() {
            match event {
                WatchEvent::Update(ports) => app.update_ports(ports),
                WatchEvent::Error(e) => eprintln!("Scan error: {}", e),
            }
        }
    }

    Ok(())
}
