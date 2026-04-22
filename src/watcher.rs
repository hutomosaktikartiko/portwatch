use anyhow::Result;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::time;

use crate::scanner::{PortInfo, Scanner};

pub enum WatchEvent {
    Update(Vec<PortInfo>),
    Error(String),
}

pub enum WatchCommand {
    RefreshNow,
}

pub async fn run_watch_loop(
    interval_secs: u64,
    min_port: u16,
    tx: mpsc::Sender<WatchEvent>,
    mut cmd_rx: mpsc::Receiver<WatchCommand>,
) -> Result<()> {
    let mut scanner = Scanner::new();
    let mut interval = time::interval(Duration::from_secs(interval_secs));

    loop {
        tokio::select! {
                _ = interval.tick() => {
                    scan_once(&mut scanner, min_port, &tx).await;
                }
                cmd = cmd_rx.recv() => {
                    match cmd {
                        Some(WatchCommand::RefreshNow) => {
                            scan_once(&mut scanner, min_port, &tx).await;
                        }
                        None => {
                            break;
                        }
                    }
                }
        }
    }

    Ok(())
}

async fn scan_once(scanner: &mut Scanner, min_port: u16, tx: &mpsc::Sender<WatchEvent>) {
    match scanner.scan(min_port) {
        Ok(ports) => {
            if tx.send(WatchEvent::Update(ports)).await.is_err() {
                return;
            }
        }
        Err(e) => {
            let _ = tx.send(WatchEvent::Error(e.to_string())).await;
        }
    }
}
