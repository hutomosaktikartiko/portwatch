mod cli;
mod scanner;

use anyhow::Ok;
use clap::Parser;
use cli::{Cli, Commands};
use colored::Colorize;
use scanner::Scanner;

fn main() -> anyhow::Result<()> {
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
        Commands::Watch { interval, tui } => {
            println!("Watch mode (interval: {}, tui: {})...", interval, tui);
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
