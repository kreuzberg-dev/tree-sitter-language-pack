use serde::Deserialize;
use std::fs;
use tree_sitter_language_pack::{DownloadManager, LanguageRegistry, ProcessConfig};

const VERSION: &str = "1.0.0-rc.7";

#[derive(Deserialize)]
struct BasicFixture {
    name: String,
    test: String,
    #[serde(default)]
    language: Option<String>,
    #[serde(default)]
    expected: Option<serde_json::Value>,
    #[serde(default)]
    expected_min: Option<usize>,
    #[serde(default)]
    expected_contains: Option<Vec<String>>,
}

#[derive(Deserialize)]
struct ProcessFixture {
    name: String,
    #[allow(dead_code)]
    test: String,
    source: String,
    config: ProcessFixtureConfig,
    expected: ProcessExpected,
}

#[derive(Deserialize)]
struct ProcessFixtureConfig {
    language: String,
    #[serde(default)]
    structure: Option<bool>,
    #[serde(default)]
    imports: Option<bool>,
    #[serde(default)]
    chunk_max_size: Option<usize>,
}

#[derive(Deserialize)]
struct ProcessExpected {
    #[serde(default)]
    language: Option<String>,
    #[serde(default)]
    structure_min: Option<usize>,
    #[serde(default)]
    imports_min: Option<usize>,
    #[serde(default)]
    metrics_total_lines_min: Option<usize>,
    #[serde(default)]
    error_count: Option<usize>,
    #[serde(default)]
    chunks_min: Option<usize>,
}

fn setup_registry() -> LanguageRegistry {
    println!("Downloading parsers for v{}...", VERSION);
    let mut dm = DownloadManager::new(VERSION).expect("Failed to create DownloadManager");

    // Download the languages we need for testing
    let needed = &["python", "javascript", "rust", "go", "ruby", "java", "c", "cpp"];
    dm.ensure_languages(needed)
        .expect("Failed to download parsers");

    let cache_dir = dm.cache_dir().to_path_buf();
    println!("Parsers cached at: {}", cache_dir.display());

    // Create registry with the download cache as an extra lib dir
    let mut registry = LanguageRegistry::new();
    registry.add_extra_libs_dir(cache_dir);
    registry
}

fn run_basic_tests(registry: &LanguageRegistry) {
    let data = fs::read_to_string("../fixtures/basic.json").expect("Failed to read basic.json");
    let fixtures: Vec<BasicFixture> =
        serde_json::from_str(&data).expect("Failed to parse basic.json");

    for fixture in &fixtures {
        match fixture.test.as_str() {
            "language_count" => {
                let count = registry.language_count();
                let min = fixture.expected_min.unwrap();
                assert!(
                    count >= min,
                    "[{}] language_count {} < expected min {}",
                    fixture.name, count, min
                );
                println!("  PASS: {} (count={})", fixture.name, count);
            }
            "has_language" => {
                let lang = fixture.language.as_ref().unwrap();
                let result = registry.has_language(lang);
                let expected = fixture.expected.as_ref().unwrap().as_bool().unwrap();
                assert_eq!(
                    result, expected,
                    "[{}] has_language({}) = {}, expected {}",
                    fixture.name, lang, result, expected
                );
                println!(
                    "  PASS: {} (has_language({})={})",
                    fixture.name, lang, result
                );
            }
            "available_languages" => {
                let langs = registry.available_languages();
                let expected_contains = fixture.expected_contains.as_ref().unwrap();
                for lang in expected_contains {
                    assert!(
                        langs.contains(lang),
                        "[{}] available_languages missing '{}'",
                        fixture.name, lang
                    );
                }
                println!(
                    "  PASS: {} (contains all expected languages)",
                    fixture.name
                );
            }
            other => panic!("Unknown test type: {}", other),
        }
    }
}

fn run_process_tests(registry: &LanguageRegistry, fixture_path: &str) {
    let data = fs::read_to_string(fixture_path)
        .unwrap_or_else(|_| panic!("Failed to read {}", fixture_path));
    let fixtures: Vec<ProcessFixture> =
        serde_json::from_str(&data).unwrap_or_else(|_| panic!("Failed to parse {}", fixture_path));

    for fixture in &fixtures {
        let mut config = ProcessConfig::new(&fixture.config.language);
        if fixture.config.structure == Some(true) {
            config.structure = true;
        }
        if fixture.config.imports == Some(true) {
            config.imports = true;
        }
        if let Some(max_size) = fixture.config.chunk_max_size {
            config = config.with_chunking(max_size);
        }

        let result = registry.process(&fixture.source, &config).unwrap_or_else(|e| {
            panic!("[{}] process() failed: {}", fixture.name, e);
        });

        if let Some(ref lang) = fixture.expected.language {
            assert_eq!(&result.language, lang, "[{}] language mismatch", fixture.name);
        }
        if let Some(min) = fixture.expected.structure_min {
            assert!(
                result.structure.len() >= min,
                "[{}] structure count {} < min {}",
                fixture.name,
                result.structure.len(),
                min
            );
        }
        if let Some(min) = fixture.expected.imports_min {
            assert!(
                result.imports.len() >= min,
                "[{}] imports count {} < min {}",
                fixture.name,
                result.imports.len(),
                min
            );
        }
        if let Some(min) = fixture.expected.chunks_min {
            assert!(
                result.chunks.len() >= min,
                "[{}] chunks count {} < min {}",
                fixture.name,
                result.chunks.len(),
                min
            );
        }

        println!("  PASS: {}", fixture.name);
    }
}

fn main() {
    let registry = setup_registry();

    println!("\n=== Basic Tests ===");
    run_basic_tests(&registry);

    println!("\n=== Process Tests ===");
    run_process_tests(&registry, "../fixtures/process.json");

    println!("\n=== Chunking Tests ===");
    run_process_tests(&registry, "../fixtures/chunking.json");

    println!("\nAll tests passed!");
}
