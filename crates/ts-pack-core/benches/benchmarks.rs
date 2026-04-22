use ahash::AHashMap;
use std::hint::black_box;

use criterion::{Criterion, criterion_group, criterion_main};
use tree_sitter_language_pack::{
    CaptureOutput, CompiledExtraction, ExtractionConfig, ExtractionPattern, ProcessConfig,
    detect_language_from_content, detect_language_from_extension, detect_language_from_path, extract_patterns,
    parse_string, process, run_query, validate_extraction,
};

// ---------------------------------------------------------------------------
// Fixture data (embedded at compile time)
// ---------------------------------------------------------------------------

const PYTHON_SMALL: &str = include_str!("../../../fixtures/bench/python/small.py");
const PYTHON_MEDIUM: &str = include_str!("../../../fixtures/bench/python/medium.py");
const PYTHON_LARGE: &str = include_str!("../../../fixtures/bench/python/large.py");

const TYPESCRIPT_SMALL: &str = include_str!("../../../fixtures/bench/typescript/small.ts");
const TYPESCRIPT_MEDIUM: &str = include_str!("../../../fixtures/bench/typescript/medium.ts");
const TYPESCRIPT_LARGE: &str = include_str!("../../../fixtures/bench/typescript/large.tsx");

const RUST_SMALL: &str = include_str!("../../../fixtures/bench/rust/small.rs");
const RUST_MEDIUM: &str = include_str!("../../../fixtures/bench/rust/medium.rs");
const RUST_LARGE: &str = include_str!("../../../fixtures/bench/rust/large.rs");

const GO_SMALL: &str = include_str!("../../../fixtures/bench/go/small.go");
const GO_MEDIUM: &str = include_str!("../../../fixtures/bench/go/medium.go");
const GO_LARGE: &str = include_str!("../../../fixtures/bench/go/large.go");

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn python_extraction_config() -> ExtractionConfig {
    let mut patterns = AHashMap::new();
    patterns.insert(
        "functions".to_string(),
        ExtractionPattern {
            query: "(function_definition name: (identifier) @name parameters: (parameters) @params) @func".to_string(),
            capture_output: CaptureOutput::Full,
            child_fields: Vec::new(),
            max_results: None,
            byte_range: None,
        },
    );
    ExtractionConfig {
        language: "python".to_string(),
        patterns,
    }
}

// ---------------------------------------------------------------------------
// 1. parse — parse_string() across 4 languages x 3 sizes
// ---------------------------------------------------------------------------

fn bench_parse(c: &mut Criterion) {
    let mut group = c.benchmark_group("parse");

    let cases: &[(&str, &str, &str)] = &[
        ("python/small", "python", PYTHON_SMALL),
        ("python/medium", "python", PYTHON_MEDIUM),
        ("python/large", "python", PYTHON_LARGE),
        ("typescript/small", "typescript", TYPESCRIPT_SMALL),
        ("typescript/medium", "typescript", TYPESCRIPT_MEDIUM),
        ("typescript/large", "tsx", TYPESCRIPT_LARGE),
        ("rust/small", "rust", RUST_SMALL),
        ("rust/medium", "rust", RUST_MEDIUM),
        ("rust/large", "rust", RUST_LARGE),
        ("go/small", "go", GO_SMALL),
        ("go/medium", "go", GO_MEDIUM),
        ("go/large", "go", GO_LARGE),
    ];

    for &(id, lang, source) in cases {
        group.bench_function(id, |b| {
            b.iter(|| parse_string(black_box(lang), black_box(source.as_bytes())).unwrap());
        });
    }

    group.finish();
}

// ---------------------------------------------------------------------------
// 2. process — all flags vs minimal, Python medium/large
// ---------------------------------------------------------------------------

fn bench_process(c: &mut Criterion) {
    let mut group = c.benchmark_group("process");

    group.bench_function("python/medium/all", |b| {
        let config = ProcessConfig::new("python").all();
        b.iter(|| process(black_box(PYTHON_MEDIUM), black_box(&config)).unwrap());
    });

    group.bench_function("python/medium/minimal", |b| {
        let config = ProcessConfig::new("python").minimal();
        b.iter(|| process(black_box(PYTHON_MEDIUM), black_box(&config)).unwrap());
    });

    group.bench_function("python/large/all", |b| {
        let config = ProcessConfig::new("python").all();
        b.iter(|| process(black_box(PYTHON_LARGE), black_box(&config)).unwrap());
    });

    group.bench_function("python/large/minimal", |b| {
        let config = ProcessConfig::new("python").minimal();
        b.iter(|| process(black_box(PYTHON_LARGE), black_box(&config)).unwrap());
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// 3. run_query — finding function definitions, Python medium
// ---------------------------------------------------------------------------

fn bench_run_query(c: &mut Criterion) {
    let mut group = c.benchmark_group("run_query");

    let tree = parse_string("python", PYTHON_MEDIUM.as_bytes()).unwrap();
    let query_src = "(function_definition name: (identifier) @fn_name) @fn_def";

    group.bench_function("python/medium/function_defs", |b| {
        b.iter(|| {
            run_query(
                black_box(&tree),
                black_box("python"),
                black_box(query_src),
                black_box(PYTHON_MEDIUM.as_bytes()),
            )
            .unwrap()
        });
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// 4. extract_oneshot — extract() one-shot, Python medium
// ---------------------------------------------------------------------------

fn bench_extract_oneshot(c: &mut Criterion) {
    let mut group = c.benchmark_group("extract_oneshot");

    let config = python_extraction_config();

    group.bench_function("python/medium", |b| {
        b.iter(|| extract_patterns(black_box(PYTHON_MEDIUM), black_box(&config)).unwrap());
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// 5. extract_compiled — CompiledExtraction::extract() amortized, Python medium
// ---------------------------------------------------------------------------

fn bench_extract_compiled(c: &mut Criterion) {
    let mut group = c.benchmark_group("extract_compiled");

    let config = python_extraction_config();
    let compiled = CompiledExtraction::compile(&config).unwrap();

    group.bench_function("python/medium", |b| {
        b.iter(|| compiled.extract(black_box(PYTHON_MEDIUM)).unwrap());
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// 6. extract_from_tree — CompiledExtraction::extract_from_tree() pre-parsed
// ---------------------------------------------------------------------------

fn bench_extract_from_tree(c: &mut Criterion) {
    let mut group = c.benchmark_group("extract_from_tree");

    let config = python_extraction_config();
    let compiled = CompiledExtraction::compile(&config).unwrap();
    let tree = parse_string("python", PYTHON_MEDIUM.as_bytes()).unwrap();

    group.bench_function("python/medium", |b| {
        b.iter(|| {
            compiled
                .extract_from_tree(black_box(&tree), black_box(PYTHON_MEDIUM.as_bytes()))
                .unwrap()
        });
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// 7. validate — validate_extraction(), Python config
// ---------------------------------------------------------------------------

fn bench_validate(c: &mut Criterion) {
    let mut group = c.benchmark_group("validate");

    let config = python_extraction_config();

    group.bench_function("python/extraction_config", |b| {
        b.iter(|| validate_extraction(black_box(&config)).unwrap());
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// 8. text_splitter — process() with chunking, Python medium
// ---------------------------------------------------------------------------

fn bench_text_splitter(c: &mut Criterion) {
    let mut group = c.benchmark_group("text_splitter");

    group.bench_function("python/medium/chunk_1000", |b| {
        let config = ProcessConfig::new("python").all().with_chunking(1000);
        b.iter(|| process(black_box(PYTHON_MEDIUM), black_box(&config)).unwrap());
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// 9. language_detection — extension, path, content detection
// ---------------------------------------------------------------------------

fn bench_language_detection(c: &mut Criterion) {
    let mut group = c.benchmark_group("language_detection");

    group.bench_function("from_extension", |b| {
        b.iter(|| detect_language_from_extension(black_box("py")));
    });

    group.bench_function("from_path", |b| {
        b.iter(|| detect_language_from_path(black_box("src/main.rs")));
    });

    group.bench_function("from_content", |b| {
        b.iter(|| detect_language_from_content(black_box("#!/usr/bin/env python3\n")));
    });

    group.finish();
}

// ---------------------------------------------------------------------------
// Criterion harness
// ---------------------------------------------------------------------------

criterion_group!(
    benches,
    bench_parse,
    bench_process,
    bench_run_query,
    bench_extract_oneshot,
    bench_extract_compiled,
    bench_extract_from_tree,
    bench_validate,
    bench_text_splitter,
    bench_language_detection,
);
criterion_main!(benches);
