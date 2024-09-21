use anyhow::{bail, Ok, Result};
use clap::{arg, command, Parser, Subcommand};

#[derive(Debug, Subcommand)]
enum HelloCommand {
    Greet {
        #[arg(short, long)]
        name: String,
    },
    Despedir {
        #[arg(short, long)]
        nombre: String,
    },
}

#[derive(Parser, Debug)]
struct Cli {
    #[command(subcommand)]
    command: Option<HelloCommand>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Some(HelloCommand::Greet { name }) => {
            println!("Hello {}!", name);
            Ok(())
        }
        Some(HelloCommand::Despedir { nombre }) => {
            println!("Hasta luego {}!", nombre);
            Ok(())
        }
        None => bail!("Command is required"),
    }
}
