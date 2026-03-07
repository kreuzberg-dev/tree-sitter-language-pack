use super::{config_path, load_config_toml, load_definitions_from_path};

pub fn run(languages: &[String], definitions_path: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let defs = load_definitions_from_path(definitions_path)?;

    // Validate all language names first
    let mut invalid = Vec::new();
    for lang in languages {
        if !defs.contains_key(lang.as_str()) {
            invalid.push(lang.as_str());
        }
    }
    if !invalid.is_empty() {
        return Err(format!("Unknown language(s): {}", invalid.join(", ")).into());
    }

    let path = config_path();

    // Create config file if it doesn't exist
    if !path.exists() {
        super::init::run(false)?;
    }

    let mut doc = load_config_toml(&path)?;

    // Ensure [languages] table exists
    if !doc.contains_key("languages") {
        doc["languages"] = toml_edit::Item::Table(toml_edit::Table::new());
    }

    // Get or create include array
    let languages_table = doc["languages"].as_table_mut().ok_or("'languages' is not a table")?;

    if !languages_table.contains_key("include") {
        languages_table["include"] = toml_edit::value(toml_edit::Array::new());
    }

    let include = languages_table["include"]
        .as_array_mut()
        .ok_or("'languages.include' is not an array")?;

    let existing: Vec<String> = include.iter().filter_map(|v| v.as_str().map(String::from)).collect();

    let mut added = Vec::new();
    for lang in languages {
        if !existing.contains(lang) {
            include.push(lang.as_str());
            added.push(lang.as_str());
        }
    }

    std::fs::write(&path, doc.to_string())?;

    if added.is_empty() {
        println!("All specified languages were already in the include list.");
    } else {
        println!("Added: {}", added.join(", "));
    }

    Ok(())
}
