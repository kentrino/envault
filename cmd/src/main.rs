use std::process::exit;
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command()]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>
}

#[derive(Subcommand)]
enum Commands {
    /// export env
    Export {
        #[arg(short, long, help = "Environment name")]
        env: String
    }
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Export { env }) => {
            println!("export {}", env);
        }
        None => {
            println!("No command specified");
            exit(1);
        }
    }
}
