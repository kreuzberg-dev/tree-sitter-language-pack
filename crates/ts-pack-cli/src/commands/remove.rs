use super::{config_path, load_config_toml};

pub fn run(languages: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let path = config_path();

    if !path.exists() {
        return Err("No language-pack.toml found. Run 'ts-pack init' first.".into());
    }

    let mut doc = load_config_toml(&path)?;

    let include = doc
        .get_mut("languages")
        .and_then(|t| t.as_table_mut())
        .and_then(|t| t.get_mut("include"))
        .and_then(|v| v.as_array_mut());

    let include = match include {
        Some(arr) => arr,
        None => {
            println!("No include list found in config. Nothing to remove.");
            return Ok(());
        }
    };

    let mut removed = Vec::new();
    for lang in languages {
        let before_len = include.len();
        include.retain(|v| v.as_str() != Some(lang.as_str()));
        if include.len() < before_len {
            removed.push(lang.as_str());
        }
    }

    std::fs::write(&path, doc.to_string())?;

    if removed.is_empty() {
        println!("None of the specified languages were in the include list.");
    } else {
        println!("Removed: {}", removed.join(", "));
    }

    Ok(())
}
