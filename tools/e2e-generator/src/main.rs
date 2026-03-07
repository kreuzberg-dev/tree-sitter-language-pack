mod fixtures;
mod generators;

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "e2e-generator", about = "Generate E2E test suites from JSON fixtures")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate test suites for one or all target languages.
    Generate {
        /// Target language: rust, python, typescript, go, java, elixir, c, or all.
        #[arg(long, default_value = "all")]
        lang: String,

        /// Path to the fixtures directory.
        #[arg(long, default_value = "fixtures")]
        fixtures: PathBuf,

        /// Output directory for generated tests.
        #[arg(long, default_value = "e2e")]
        output: PathBuf,
    },

    /// List all loaded fixtures.
    List {
        /// Path to the fixtures directory.
        #[arg(long, default_value = "fixtures")]
        fixtures: PathBuf,
    },

    /// Generate smoke fixtures from language_definitions.json.
    GenerateSmokeFixtures {
        /// Path to language_definitions.json.
        #[arg(long)]
        definitions: PathBuf,

        /// Output directory for generated fixture files.
        #[arg(long, default_value = "fixtures/smoke")]
        output: PathBuf,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Generate {
            lang,
            fixtures: fixtures_dir,
            output,
        } => {
            let all_fixtures = match fixtures::load_fixtures(&fixtures_dir) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error loading fixtures: {e}");
                    std::process::exit(1);
                }
            };

            eprintln!("Loaded {} fixtures from {}", all_fixtures.len(), fixtures_dir.display());

            let targets = if lang == "all" {
                generators::ALL_TARGETS.to_vec()
            } else {
                vec![lang.as_str()]
            };

            for target in &targets {
                if let Err(e) = generators::generate_for_lang(target, &all_fixtures, &output) {
                    eprintln!("Error generating {target} tests: {e}");
                    std::process::exit(1);
                }
            }

            eprintln!("Generated tests in {}", output.display());
        }

        Commands::List { fixtures: fixtures_dir } => {
            let all_fixtures = match fixtures::load_fixtures(&fixtures_dir) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Error loading fixtures: {e}");
                    std::process::exit(1);
                }
            };

            println!("{:<30} {:<20} DESCRIPTION", "ID", "CATEGORY");
            println!("{}", "-".repeat(80));

            for fixture in &all_fixtures {
                println!("{:<30} {:<20} {}", fixture.id, fixture.category, fixture.description);
            }

            println!("\nTotal: {} fixtures", all_fixtures.len());
        }

        Commands::GenerateSmokeFixtures { definitions, output } => {
            let content = match std::fs::read_to_string(&definitions) {
                Ok(c) => c,
                Err(e) => {
                    eprintln!("Error reading {}: {e}", definitions.display());
                    std::process::exit(1);
                }
            };

            let defs: serde_json::Value = match serde_json::from_str(&content) {
                Ok(v) => v,
                Err(e) => {
                    eprintln!("Error parsing {}: {e}", definitions.display());
                    std::process::exit(1);
                }
            };

            let obj = match defs.as_object() {
                Some(o) => o,
                None => {
                    eprintln!("Expected top-level JSON object in {}", definitions.display());
                    std::process::exit(1);
                }
            };

            if let Err(e) = std::fs::create_dir_all(&output) {
                eprintln!("Error creating output dir: {e}");
                std::process::exit(1);
            }

            let mut count = 0;
            let source_snippets = default_source_snippets();

            for lang_name in obj.keys() {
                let source_code = source_snippets.get(lang_name.as_str()).copied().unwrap_or("x");

                let fixture = serde_json::json!({
                    "id": format!("smoke_{lang_name}"),
                    "category": "smoke",
                    "description": format!("Smoke test: load {lang_name} and parse a simple snippet"),
                    "language": lang_name,
                    "source_code": source_code,
                    "assertions": {
                        "tree_not_null": true,
                        "root_child_count_min": 1
                    },
                    "tags": ["smoke"]
                });

                let path = output.join(format!("{lang_name}.json"));
                let json = serde_json::to_string_pretty(&fixture).unwrap();

                if let Err(e) = std::fs::write(&path, format!("{json}\n")) {
                    eprintln!("Error writing {}: {e}", path.display());
                    std::process::exit(1);
                }

                count += 1;
            }

            eprintln!("Generated {count} smoke fixtures in {}", output.display());
        }
    }
}

/// Returns a map of language names to simple source code snippets.
fn default_source_snippets() -> std::collections::HashMap<&'static str, &'static str> {
    let mut m = std::collections::HashMap::new();
    m.insert("python", "print('hello')");
    m.insert("rust", "fn main() {}");
    m.insert("javascript", "console.log('hello');");
    m.insert("typescript", "const x: number = 42;");
    m.insert("go", "package main");
    m.insert("c", "int main() { return 0; }");
    m.insert("cpp", "int main() { return 0; }");
    m.insert("java", "class Main {}");
    m.insert("ruby", "puts 'hello'");
    m.insert("html", "<p>hello</p>");
    m.insert("css", "body { color: red; }");
    m.insert("json", "{\"key\": \"value\"}");
    m.insert("yaml", "key: value");
    m.insert("toml", "key = \"value\"");
    m.insert("bash", "echo hello");
    m.insert("lua", "print('hello')");
    m.insert("php", "<?php echo 'hello'; ?>");
    m.insert("swift", "print(\"hello\")");
    m.insert("kotlin", "fun main() {}");
    m.insert("scala", "object Main");
    m.insert("haskell", "main = putStrLn \"hello\"");
    m.insert("elixir", "IO.puts(\"hello\")");
    m.insert("erlang", "main() -> ok.");
    m.insert("ocaml", "let () = print_endline \"hello\"");
    m.insert("sql", "SELECT 1;");
    m.insert("r", "print('hello')");
    m.insert("perl", "print 'hello';");
    m.insert("zig", "pub fn main() void {}");
    m
}
