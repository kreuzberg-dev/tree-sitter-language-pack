use super::{config_path, load_definitions_from_path};
use ts_pack_core::config::Config;

pub fn run(installed: bool, definitions_path: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let defs = load_definitions_from_path(definitions_path)?;

    let filter: Option<Vec<String>> = if installed {
        let cfg_path = config_path();
        if cfg_path.exists() {
            let config = Config::load(&cfg_path).map_err(|e| format!("Failed to load config: {e}"))?;
            if config.languages.include.is_empty() {
                None
            } else {
                Some(config.languages.include)
            }
        } else {
            return Err("No language-pack.toml found. Run 'ts-pack init' first.".into());
        }
    } else {
        None
    };

    println!("{:<25} {:<55} PINNED", "NAME", "REPO");
    println!("{}", "-".repeat(90));

    let mut count = 0;
    for (name, def) in &defs {
        if let Some(ref include) = filter
            && !include.contains(name)
        {
            continue;
        }

        let pinned = if def.rev.is_some() { "yes" } else { "no" };
        println!("{:<25} {:<55} {}", name, def.repo, pinned);
        count += 1;
    }

    println!("\nTotal: {count} languages");
    Ok(())
}
