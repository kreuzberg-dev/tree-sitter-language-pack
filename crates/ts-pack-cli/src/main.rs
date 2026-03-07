mod commands;

use clap::Parser;
use commands::{Cli, Commands};
use std::process;

fn main() {
    let cli = Cli::parse();

    let result = match cli.command {
        Commands::Init { force } => commands::init::run(force),
        Commands::List {
            installed,
            ref definitions,
        } => commands::list::run(installed, definitions.as_deref()),
        Commands::Add {
            ref languages,
            ref definitions,
        } => commands::add::run(languages, definitions.as_deref()),
        Commands::Remove { ref languages } => commands::remove::run(languages),
        Commands::Info {
            ref language,
            ref definitions,
        } => commands::info::run(language, definitions.as_deref()),
        Commands::Build { ref definitions } => commands::build::run(definitions.as_deref()),
    };

    if let Err(e) = result {
        eprintln!("Error: {e}");
        process::exit(1);
    }
}
