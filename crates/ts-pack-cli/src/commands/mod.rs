pub mod add;
pub mod build;
pub mod info;
pub mod init;
pub mod list;
pub mod remove;

use clap::{Parser, Subcommand};
use std::path::{Path, PathBuf};
use ts_pack_core::definitions::{LanguageDefinitions, load_definitions};

#[derive(Parser)]
#[command(name = "ts-pack", about = "Manage tree-sitter language grammars")]
#[command(version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Create a new language-pack.toml configuration file
    Init {
        /// Overwrite existing language-pack.toml
        #[arg(long)]
        force: bool,
    },
    /// List available languages
    List {
        /// Show only languages in the current config's include list
        #[arg(long)]
        installed: bool,
        /// Path to language_definitions.json
        #[arg(long)]
        definitions: Option<String>,
    },
    /// Add languages to the configuration
    Add {
        /// Language names to add
        #[arg(required = true)]
        languages: Vec<String>,
        /// Path to language_definitions.json
        #[arg(long)]
        definitions: Option<String>,
    },
    /// Remove languages from the configuration
    Remove {
        /// Language names to remove
        #[arg(required = true)]
        languages: Vec<String>,
    },
    /// Show details about a language
    Info {
        /// Language name
        language: String,
        /// Path to language_definitions.json
        #[arg(long)]
        definitions: Option<String>,
    },
    /// Show build instructions and configured languages
    Build {
        /// Path to language_definitions.json
        #[arg(long)]
        definitions: Option<String>,
    },
}

const CONFIG_FILENAME: &str = "language-pack.toml";

pub fn load_definitions_from_path(
    definitions_path: Option<&str>,
) -> Result<LanguageDefinitions, Box<dyn std::error::Error>> {
    let path = match definitions_path {
        Some(p) => PathBuf::from(p),
        None => find_definitions_file()?,
    };
    let content = std::fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read definitions file '{}': {e}", path.display()))?;
    let defs = load_definitions(&content)?;
    Ok(defs)
}

fn find_definitions_file() -> Result<PathBuf, Box<dyn std::error::Error>> {
    // Check CWD/sources/language_definitions.json
    let cwd = std::env::current_dir()?;
    let candidate = cwd.join("sources").join("language_definitions.json");
    if candidate.exists() {
        return Ok(candidate);
    }
    // Check if there's a language-pack.toml with a definitions path
    let config_path = cwd.join(CONFIG_FILENAME);
    if config_path.exists() {
        let config =
            ts_pack_core::config::Config::load(&config_path).map_err(|e| format!("Failed to load config: {e}"))?;
        if let Some(def_path) = config.language_pack.definitions {
            let p = PathBuf::from(&def_path);
            if p.exists() {
                return Ok(p);
            }
        }
    }
    Err("Could not find language_definitions.json. Use --definitions to specify the path.".into())
}

pub fn config_path() -> PathBuf {
    std::env::current_dir()
        .unwrap_or_else(|_| PathBuf::from("."))
        .join(CONFIG_FILENAME)
}

pub fn load_config_toml(path: &Path) -> Result<toml_edit::DocumentMut, Box<dyn std::error::Error>> {
    let content = std::fs::read_to_string(path).map_err(|e| format!("Failed to read '{}': {e}", path.display()))?;
    let doc: toml_edit::DocumentMut = content
        .parse()
        .map_err(|e| format!("Failed to parse '{}': {e}", path.display()))?;
    Ok(doc)
}
