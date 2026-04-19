use anyhow::Result;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;

use crate::scanner::{PortInfo, Scanner};

pub enum WatchEvent {
    Update(Vec<PortInfo>),
    Error(String),
    Stop,
}

pub async fn run_watch_loop(
    interval_secs: u64,
    min_port: u16,
    tx: mpsc::Sender<WatchEvent>,
) -> Result<()> {
    let mut scanner = Scanner::new();
    let mut interval = time::interval(Duration::from_secs(interval_secs));

    loop {
        interval.tick().await;

        match scanner.scan(min_port) {
            Ok(ports) => {
                // Send update
                if tx.send(WatchEvent::Update(ports)).await.is_err() {
                    break;
                }
            }
            Err(e) => {
                let _ = tx.send(WatchEvent::Error(e.to_string())).await;
            }
        }
    }

    Ok(())
}
