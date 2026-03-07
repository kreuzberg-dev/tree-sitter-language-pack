use super::load_definitions_from_path;
use std::path::PathBuf;

pub fn run(language: &str, definitions_path: Option<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let defs = load_definitions_from_path(definitions_path)?;

    let def = defs
        .get(language)
        .ok_or_else(|| format!("Language '{language}' not found in definitions"))?;

    println!("Language: {language}");
    println!("Repo:     {}", def.repo);
    println!("Rev:      {}", def.rev.as_deref().unwrap_or("(not pinned)"));
    println!("Branch:   {}", def.branch.as_deref().unwrap_or("(default)"));
    println!("Directory: {}", def.directory.as_deref().unwrap_or("(root)"));
    println!(
        "Generate: {}",
        def.generate.map_or("no", |g| if g { "yes" } else { "no" })
    );
    println!(
        "ABI version: {}",
        def.abi_version.map_or("(default)".to_string(), |v| v.to_string())
    );

    // Check if parser sources exist locally
    let cwd = std::env::current_dir()?;
    let parser_dir = cwd.join("parsers").join(format!("tree-sitter-{language}"));
    let alt_parser_dir = cwd.join("parsers").join(language);
    // Also check vendors/ for backward compatibility
    let vendor_dir = cwd.join("vendors").join(format!("tree-sitter-{language}"));

    let local_path = find_existing(&[parser_dir, alt_parser_dir, vendor_dir]);
    println!(
        "Local:    {}",
        match &local_path {
            Some(p) => format!("present ({})", p.display()),
            None => "not found".to_string(),
        }
    );

    Ok(())
}

fn find_existing(paths: &[PathBuf]) -> Option<PathBuf> {
    paths.iter().find(|p| p.exists()).cloned()
}
