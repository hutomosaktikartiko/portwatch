use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "portwatch")]
#[command(name = "Monitor open ports and their processes")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    // Scan and display open ports
    Scan {
        #[arg(short, long, default_value_t = 0)]
        min_port: u16,

        #[arg(short, long)]
        json: bool,
    },

    // Watch mode - automatically refresh every N seconds
    Watch {
        #[arg(short, long, default_value_t = 2)]
        interval: u64,

        #[arg(short, long)]
        tui: bool,
    },
}
