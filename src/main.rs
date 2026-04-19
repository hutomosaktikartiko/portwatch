mod cli;
mod scanner;
mod watcher;

use anyhow::Ok;
use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use scanner::Scanner;

use crate::watcher::WatchEvent;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Scan { min_port, json } => {
            let mut scanner = Scanner::new();
            let ports = scanner.scan(min_port)?;

            if json {
                println!("{}", serde_json::to_string_pretty(&ports)?);
            } else {
                print_table(&ports);
            }

            println!("Scanning ports (min: {}, json: {})...", min_port, json);
        }
        Commands::Watch { interval, tui } if !tui => {
            let (tx, mut rx) = tokio::sync::mpsc::channel(32);

            // Spawn watcher as background task
            tokio::spawn(async move {
                watcher::run_watch_loop(interval, 0, tx).await.unwrap();
            });

            // Handle ctrl+c
            let ctrl_c = tokio::signal::ctrl_c();
            tokio::pin!(ctrl_c);

            loop {
                tokio::select! {
                    Some(event) = rx.recv() => {
                        match event {
                            WatchEvent::Update(ports) => {
                                print!("\x1B[2J\x1B[1;1H");
                                println!("🔍 refreshing every {}s\n", interval);

                                print_table(&ports);
                            }
                            WatchEvent::Error(e) => {
                                eprintln!("Error: {}", e);
                            }
                            _ => {}
                        }
                    }
                    _ = &mut ctrl_c => {
                        println!("\nStoppping...");
                        break;
                    }
                }
            }
        }
        _ => {
            println!("Unsupported command: {:?}...", cli.command);
        }
    }

    Ok(())
}

fn print_table(ports: &[scanner::PortInfo]) {
    println!("{}", "-".repeat(60).bright_black());
    println!(
        "{:<8} {:<8} {:<10} {}",
        "PORT".bold(),
        "PROTO".bold(),
        "PID".bold(),
        "PROCESS".bold()
    );
    println!("{}", "-".repeat(60).bright_black());

    for p in ports {
        let pid_str = p.pid.map(|id| id.to_string()).unwrap_or("-".to_string());
        let proc_str = p.process_name.as_deref().unwrap_or("unkwon");
        println!(
            "{:<8} {:<8} {:<10} {}",
            p.port.to_string().cyan(),
            p.protocol.yellow(),
            pid_str.bright_black(),
            proc_str.green()
        );
    }
    println!("{}", "-".repeat(60).bright_black());
    println!("{} ports(s) found", ports.len());
}
