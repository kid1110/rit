use clap::{Parser, Subcommand};
use rit::commands;
use rit::models;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Init,
}
fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Init) => {
            let _ = commands::init();
        }
        None => {}
    }
}
