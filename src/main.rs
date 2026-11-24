mod cli;

use clap::{CommandFactory, FromArgMatches};
use cli::Cli;
use std::process;

#[tokio::main]
async fn main() {
    let mut command = Cli::command();
    if !cli::show_env_values_requested() {
        command = cli::hide_all_env_values(command);
    }
    let cli_args = Cli::from_arg_matches(&command.get_matches()).unwrap_or_else(|e| e.exit());

    match cli_args.run().await {
        Ok(str) => {
            if let Some(s) = str {
                println!("{s}");
            }
        }
        Err(err) => {
            eprintln!("{err:?}");
            process::exit(1);
        }
    }
}
