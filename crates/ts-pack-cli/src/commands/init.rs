use super::config_path;

const TEMPLATE: &str = r#"# tree-sitter language pack configuration
# See: https://github.com/Goldziher/tree-sitter-language-pack

[language-pack]
# cache_dir = ".ts-cache"
# definitions = "sources/language_definitions.json"

[languages]
# List specific languages to include (empty = all languages)
include = []

# List languages to exclude
exclude = []
"#;

pub fn run(force: bool) -> Result<(), Box<dyn std::error::Error>> {
    let path = config_path();

    if path.exists() && !force {
        return Err(format!("'{}' already exists. Use --force to overwrite.", path.display()).into());
    }

    std::fs::write(&path, TEMPLATE)?;
    println!("Created {}", path.display());
    Ok(())
}
