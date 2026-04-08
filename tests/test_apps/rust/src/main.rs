use serde::Deserialize;
use std::fs;
use tree_sitter_language_pack::{
    cache_dir, configure, downloaded_languages, manifest_languages, parse_string,
    tree_has_error_nodes, DownloadManager, LanguageRegistry, PackConfig, ProcessConfig,
};

const VERSION: &str = "1.5.0";

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

    let cache = dm.cache_dir().to_path_buf();
    println!("Parsers cached at: {}", cache.display());

    // Also configure the global registry so parse_string() can find downloaded parsers
    let config = PackConfig {
        cache_dir: Some(cache.clone()),
        languages: None,
        groups: None,
    };
    tree_sitter_language_pack::configure(&config).expect("Failed to configure global registry");

    // Create registry with the download cache as an extra lib dir
    let registry = LanguageRegistry::new();
    registry.add_extra_libs_dir(cache);
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

fn run_download_api_tests() {
    // Test: download() is callable (already done in setup, verify via downloaded_languages)
    let langs = downloaded_languages();
    assert!(
        langs.iter().any(|l| l == "python"),
        "downloaded_languages() should include 'python' after setup"
    );
    println!("  PASS: download_callable (download completed, python present)");

    // Test: downloaded_languages() returns a list
    let dl = downloaded_languages();
    assert!(
        !dl.is_empty(),
        "downloaded_languages() should return non-empty list after download"
    );
    println!("  PASS: downloaded_languages_returns_list (count={})", dl.len());

    // Test: cache_dir() returns a string
    let dir = cache_dir().expect("cache_dir() should succeed");
    let dir_str = dir.to_string_lossy();
    assert!(!dir_str.is_empty(), "cache_dir() should return non-empty path");
    println!("  PASS: cache_dir_returns_string ({})", dir_str);

    // Test: manifest_languages() returns 305 languages (network)
    match manifest_languages() {
        Ok(manifest) => {
            assert!(
                manifest.len() >= 170,
                "manifest_languages() count {} < expected 170",
                manifest.len()
            );
            println!(
                "  PASS: manifest_languages_returns_170_plus (count={})",
                manifest.len()
            );
        }
        Err(e) => {
            println!("  SKIP: manifest_languages_returns_170_plus (network unavailable: {})", e);
        }
    }
}

fn run_parse_validation_tests() {
    // Test: parse_string with valid Python code
    let python_code = b"def hello(): pass\n";
    let result = parse_string("python", python_code);
    assert!(
        result.is_ok(),
        "parse_string('python', code) should succeed"
    );
    let tree = result.unwrap();
    let root_node = tree.root_node();

    // Validate root node type is "module"
    assert_eq!(
        root_node.kind(),
        "module",
        "Root node type should be 'module' for Python code"
    );
    println!("  PASS: python_parse_root_type_is_module");

    // Validate child count >= 1
    let child_count = root_node.child_count();
    assert!(
        child_count >= 1,
        "Root node should have at least 1 child, got {}",
        child_count
    );
    println!("  PASS: python_parse_child_count_min (count={})", child_count);

    // Validate no error nodes
    let has_errors = tree_has_error_nodes(&tree);
    assert!(
        !has_errors,
        "Parse tree should not contain error nodes for valid code"
    );
    println!("  PASS: python_parse_no_error_nodes");

    // Test: parse_string with invalid language should fail
    let invalid_lang_result = parse_string("nonexistent_xyz_123", b"some code");
    assert!(
        invalid_lang_result.is_err(),
        "parse_string() with invalid language should return Err"
    );
    println!("  PASS: parse_string_invalid_language_returns_err");
}

fn run_error_handling_tests(registry: &LanguageRegistry) {
    // Test: invalid language throws an error from process()
    let config = ProcessConfig::new("nonexistent_xyz_123");
    let result = registry.process("some code", &config);
    assert!(
        result.is_err(),
        "process() with invalid language should return Err"
    );
    println!("  PASS: invalid_language_throws");

    // Test: has_language returns false for nonexistent language
    let found = registry.has_language("nonexistent_xyz_123");
    assert!(!found, "has_language(nonexistent) should return false");
    println!("  PASS: has_language_nonexistent_false");
}

fn main() {
    let registry = setup_registry();

    println!("\n=== Basic Tests ===");
    run_basic_tests(&registry);

    println!("\n=== Parse Validation Tests ===");
    run_parse_validation_tests();

    println!("\n=== Process Tests ===");
    run_process_tests(&registry, "../fixtures/process.json");

    println!("\n=== Chunking Tests ===");
    run_process_tests(&registry, "../fixtures/chunking.json");

    println!("\n=== Download API Tests ===");
    run_download_api_tests();

    println!("\n=== Error Handling Tests ===");
    run_error_handling_tests(&registry);

    println!("\nAll tests passed!");
}
