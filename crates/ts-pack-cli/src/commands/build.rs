use super::{config_path, load_definitions_from_path};
use ts_pack_core::config::Config;

pub fn run(definitions_path: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let defs = load_definitions_from_path(definitions_path)?;
    let total = defs.len();

    let cfg_path = config_path();
    let config = if cfg_path.exists() {
        Some(Config::load(&cfg_path).map_err(|e| format!("Failed to load config: {e}"))?)
    } else {
        None
    };

    let included: Vec<&str> = match &config {
        Some(cfg) if !cfg.languages.include.is_empty() => cfg.languages.include.iter().map(|s| s.as_str()).collect(),
        _ => defs.keys().map(|s| s.as_str()).collect(),
    };

    println!("Build Instructions");
    println!("==================\n");

    println!("Available languages: {total}");
    println!("Configured languages: {}\n", included.len());

    if config.is_some() && included.len() < total {
        println!("Configured languages:");
        for lang in &included {
            println!("  - {lang}");
        }
        println!();
    }

    println!("Option 1: Build with cargo features");
    println!("------------------------------------");
    if included.len() == total {
        println!("  cargo build -p ts-pack-core --features all\n");
    } else {
        let features: Vec<String> = included.iter().map(|l| format!("lang-{l}")).collect();
        println!("  cargo build -p ts-pack-core --features \"{}\"\n", features.join(","));
    }

    println!("Option 2: Build with TSLP_LANGUAGES env var");
    println!("--------------------------------------------");
    println!(
        "  TSLP_LANGUAGES=\"{}\" cargo build -p ts-pack-core\n",
        included.join(",")
    );

    println!("Option 3: Build everything");
    println!("--------------------------");
    println!("  cargo build -p ts-pack-core --features all");

    Ok(())
}
