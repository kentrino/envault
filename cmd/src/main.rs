mod util;

use clap::{Parser, Subcommand};
use config::config::Config;
use std::process::exit;

#[derive(Parser)]
#[command(name = "envalut")]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    /// export env
    Export {
        #[arg(short = 'e', long, help = "Environment name")]
        env: String,

        #[arg(short = 'c', long = "enc", help = "Encrypted file")]
        enc: String,
    },
    #[command(about = "encrypt environment variable", name = "encrypt")]
    Encrypt {
        #[arg(short, long, help = "Environment name")]
        env: Option<String>,

        #[arg(short = 'r', long = "raw", help = "Raw file", value_name = "RAW")]
        raw: String,

        #[arg(short = 'c', long = "enc", help = "Encrypted file")]
        enc: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Some(Commands::Export { env, enc }) => {
            let cfg = Config::load(Some(&enc), None).unwrap();
            let result = cfg.export(&env).unwrap();
            println!("{}", result);
        }
        Some(Commands::Encrypt { env, raw, enc }) => {
            let mut cfg = Config::load(None, Some(&raw)).unwrap();
            cfg.apply(env).unwrap();
            let out = enc.unwrap_or_else(|| util::out_path(&raw));
            cfg.save(&out).unwrap();
        }
        None => {
            println!("No command specified");
            exit(1);
        }
    }
}
