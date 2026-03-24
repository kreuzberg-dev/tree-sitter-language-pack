use serde::Deserialize;
use std::collections::BTreeMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct LanguageDefinition {
    #[allow(dead_code)]
    repo: String,
    #[allow(dead_code)]
    rev: Option<String>,
    #[allow(dead_code)]
    branch: Option<String>,
    #[allow(dead_code)]
    directory: Option<String>,
    #[allow(dead_code)]
    generate: Option<bool>,
    #[allow(dead_code)]
    abi_version: Option<u32>,
    #[serde(default)]
    extensions: Vec<String>,
    #[serde(default)]
    ambiguous: BTreeMap<String, Vec<String>>,
    /// Override for the C symbol name when it differs from the language name.
    /// E.g. language "csharp" exports `tree_sitter_c_sharp()`.
    #[serde(default)]
    c_symbol: Option<String>,
}

fn find_project_root() -> PathBuf {
    if let Ok(root) = env::var("PROJECT_ROOT") {
        return PathBuf::from(root);
    }

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let mut dir = manifest_dir.as_path();
    loop {
        if dir.join("sources/language_definitions.json").exists() {
            return dir.to_path_buf();
        }
        match dir.parent() {
            Some(parent) => dir = parent,
            // When installed from crates.io, sources/ won't exist — fall back to manifest dir
            None => return manifest_dir,
        }
    }
}

fn selected_languages(definitions: &BTreeMap<String, LanguageDefinition>) -> Vec<String> {
    // Check TSLP_LANGUAGES env var first
    if let Ok(langs) = env::var("TSLP_LANGUAGES") {
        let selected: Vec<String> = langs.split(',').map(|s| s.trim().to_string()).collect();
        // Validate language names
        for name in &selected {
            if !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                panic!(
                    "Invalid language name in TSLP_LANGUAGES: '{}'. Only alphanumeric and underscore characters are allowed.",
                    name
                );
            }
            if !definitions.contains_key(name) {
                println!(
                    "cargo:warning=Language '{}' from TSLP_LANGUAGES not found in language_definitions.json",
                    name
                );
            }
        }
        return selected;
    }

    // Check Cargo features: lang-* features
    let mut selected = Vec::new();
    for name in definitions.keys() {
        let feature_env = format!("CARGO_FEATURE_LANG_{}", name.to_uppercase().replace('-', "_"));
        if env::var(&feature_env).is_ok() {
            selected.push(name.clone());
        }
    }

    if selected.is_empty() {
        return Vec::new();
    }

    selected
}

/// Get the target OS, using CARGO_CFG_TARGET_OS for cross-compilation correctness.
fn target_os() -> String {
    env::var("CARGO_CFG_TARGET_OS").unwrap_or_else(|_| {
        if cfg!(target_os = "macos") {
            "macos".to_string()
        } else if cfg!(target_os = "windows") {
            "windows".to_string()
        } else {
            "linux".to_string()
        }
    })
}

/// Get shared library filename components for the target OS.
fn shared_lib_components(target_os: &str) -> (&'static str, &'static str) {
    match target_os {
        "macos" | "ios" => ("lib", "dylib"),
        "windows" => ("", "dll"),
        _ => ("lib", "so"),
    }
}

/// Compile a single language parser into a shared library (.so/.dylib/.dll).
fn compile_parser_dynamic(name: &str, parser_dir: &Path, output_dir: &Path) -> bool {
    let src_dir = parser_dir.join("src");
    let parser_c = src_dir.join("parser.c");

    if !parser_c.exists() {
        println!(
            "cargo:warning=Skipping language '{}': parser.c not found at {}",
            name,
            parser_c.display()
        );
        return false;
    }

    let mut c_sources = vec![parser_c];
    let scanner_c = src_dir.join("scanner.c");
    if scanner_c.exists() {
        c_sources.push(scanner_c);
    }

    let scanner_cc = src_dir.join("scanner.cc");

    let mut includes = vec![src_dir.clone()];
    let common_dir = parser_dir.join("common");
    if common_dir.exists() {
        includes.push(common_dir);
    }

    let lib_name = format!("tree_sitter_{name}");
    let os = target_os();
    let (prefix, ext) = shared_lib_components(&os);
    let output_path = output_dir.join(format!("{prefix}{lib_name}.{ext}"));

    let compiler = cc::Build::new().get_compiler();
    let is_msvc = compiler.is_like_msvc();

    let mut cmd = compiler.to_command();

    if is_msvc {
        // MSVC flags
        cmd.arg("/std:c11");
        cmd.arg("/utf-8");
        cmd.arg("/O2");
        cmd.arg("/wd4244");
        cmd.arg("/wd4566");
        cmd.arg("/wd4819");
        for inc in &includes {
            cmd.arg(format!("/I{}", inc.display()));
        }
        for src in &c_sources {
            cmd.arg(src);
        }
        // C++ scanner for MSVC
        if scanner_cc.exists() {
            cmd.arg("/TP"); // Treat next file as C++
            cmd.arg(&scanner_cc);
        }
        cmd.arg("/LD"); // Create DLL
        cmd.arg(format!("/Fe:{}", output_path.display()));
    } else {
        // GCC/Clang flags
        cmd.arg("-std=c11");
        cmd.arg("-O2");
        cmd.arg("-fPIC");
        for inc in &includes {
            cmd.arg(format!("-I{}", inc.display()));
        }
        for src in &c_sources {
            cmd.arg(src);
        }
        // C++ scanner: compile separately and link
        if scanner_cc.exists() {
            // For shared lib with mixed C/C++, we need to handle this carefully.
            // Compile scanner.cc to an object file first, then link everything.
            let scanner_obj = output_dir.join(format!("{name}_scanner.o"));
            let cpp_compiler = cc::Build::new().cpp(true).get_compiler();
            let mut cpp_cmd = cpp_compiler.to_command();
            cpp_cmd.arg("-c");
            cpp_cmd.arg("-fPIC");
            cpp_cmd.arg("-O2");
            for inc in &includes {
                cpp_cmd.arg(format!("-I{}", inc.display()));
            }
            cpp_cmd.arg(&scanner_cc);
            cpp_cmd.arg("-o");
            cpp_cmd.arg(&scanner_obj);
            let cpp_status = cpp_cmd.status();
            match cpp_status {
                Ok(s) if s.success() => {
                    cmd.arg(&scanner_obj);
                }
                Ok(s) => {
                    println!(
                        "cargo:warning=Failed to compile C++ scanner for '{}': exit code {:?}",
                        name,
                        s.code()
                    );
                    return false;
                }
                Err(e) => {
                    println!("cargo:warning=Failed to run C++ compiler for '{}': {}", name, e);
                    return false;
                }
            }
        }

        if os == "macos" || os == "ios" {
            cmd.arg("-dynamiclib");
        } else {
            cmd.arg("-shared");
        }
        cmd.arg("-o");
        cmd.arg(&output_path);
    }

    let status = cmd.status();
    match status {
        Ok(s) if s.success() => true,
        Ok(s) => {
            println!(
                "cargo:warning=Failed to compile shared library for '{}': exit code {:?}",
                name,
                s.code()
            );
            false
        }
        Err(e) => {
            println!("cargo:warning=Failed to run compiler for '{}': {}", name, e);
            false
        }
    }
}

/// Find the wasi-sysroot include path for wasm32 cross-compilation.
/// Checks WASI_SYSROOT env, then common Homebrew/system paths.
fn find_wasi_sysroot() -> Option<PathBuf> {
    if let Ok(sysroot) = env::var("WASI_SYSROOT") {
        let p = PathBuf::from(sysroot);
        if p.exists() {
            return Some(p);
        }
    }

    // Homebrew wasi-libc paths
    let candidates = [
        "/opt/homebrew/share/wasi-sysroot",
        "/usr/local/share/wasi-sysroot",
        "/opt/wasi-sdk/share/wasi-sysroot",
    ];
    for candidate in &candidates {
        let p = PathBuf::from(candidate);
        if p.exists() {
            return Some(p);
        }
    }

    // Homebrew Cellar (version-independent glob)
    if let Ok(entries) = fs::read_dir("/opt/homebrew/Cellar/wasi-libc") {
        for entry in entries.flatten() {
            let sysroot = entry.path().join("share/wasi-sysroot");
            if sysroot.exists() {
                return Some(sysroot);
            }
        }
    }

    None
}

/// Optimize cc::Build for wasm32 targets to reduce memory usage on CI runners.
/// Disables debug info to reduce object file sizes.
fn apply_wasm32_optimizations(build: &mut cc::Build) {
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default() == "wasm32" {
        build.cargo_warnings(false);
        build.debug(false);
        build.opt_level(2);
    }
}

/// Apply wasi-sysroot includes to a cc::Build for wasm32 targets.
///
/// Use `-isystem` to add the wasm32-wasi include dir which has stdlib.h etc.
/// Avoid `--sysroot` which pulls in wasi/api.h through stdio.h and fails
/// for wasm32-unknown-unknown targets.
fn apply_wasm32_sysroot(build: &mut cc::Build) {
    if env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default() != "wasm32" {
        return;
    }

    if let Some(sysroot) = find_wasi_sysroot() {
        let wasi_include = sysroot.join("include/wasm32-wasi");
        if wasi_include.exists() {
            // Define __wasi__ for C compilation only so wasi/api.h's platform
            // guard passes. This doesn't affect Rust code or wasm-bindgen output.
            // Parsers only use basic C headers (malloc, string), not WASI APIs.
            build.define("__wasi__", None);
            build.flag(format!("-isystem{}", wasi_include.display()));
        }
    } else {
        println!(
            "cargo:warning=wasm32 target detected but no wasi-sysroot found. \
                  Install wasi-libc (brew install wasi-libc) or set WASI_SYSROOT env var."
        );
    }
}

/// Compile a parser statically and link it into the main binary.
///
/// Compiles parser.c and scanner.c/cc separately to avoid symbol collisions
/// when statically linking multiple grammars. Scanner functions (`scan`,
/// `deserialize`, `serialize`, `scan_comment`) are prefixed with the language
/// name via C preprocessor defines.
fn compile_parser_static(name: &str, parser_dir: &Path) -> bool {
    let src_dir = parser_dir.join("src");
    let parser_c = src_dir.join("parser.c");
    let common_dir = parser_dir.join("common");

    // Step 1: Compile parser.c (no symbol conflicts — each has unique tree_sitter_{name})
    let mut build = cc::Build::new();
    build
        .include(&src_dir)
        .file(&parser_c)
        .define("TREE_SITTER_HIDE_SYMBOLS", None)
        .flag_if_supported("-fvisibility=hidden")
        .warnings(false);
    build.std("c11");
    apply_wasm32_sysroot(&mut build);
    apply_wasm32_optimizations(&mut build);
    if common_dir.exists() {
        build.include(&common_dir);
    }

    if let Err(e) = build.try_compile(&format!("tree_sitter_{name}_parser")) {
        println!("cargo:warning=Failed to compile parser for '{}': {}", name, e);
        return false;
    }

    // Step 2: Compile scanner.c separately with symbol prefixing to avoid collisions.
    // Many grammars define unprefixed functions like `scan`, `deserialize`, `serialize`,
    // `scan_comment` which collide when multiple grammars are linked into one binary.
    let scanner_c = src_dir.join("scanner.c");
    if scanner_c.exists() {
        let mut scanner_build = cc::Build::new();
        scanner_build
            .include(&src_dir)
            .file(&scanner_c)
            .define("TREE_SITTER_HIDE_SYMBOLS", None)
            .define("scan", &*format!("tree_sitter_{name}_ext_scan"))
            .define("deserialize", &*format!("tree_sitter_{name}_ext_deserialize"))
            .define("serialize", &*format!("tree_sitter_{name}_ext_serialize"))
            .define("scan_comment", &*format!("tree_sitter_{name}_ext_scan_comment"))
            .flag_if_supported("-fvisibility=hidden")
            .warnings(false);
        scanner_build.std("c11");
        apply_wasm32_sysroot(&mut scanner_build);
        apply_wasm32_optimizations(&mut scanner_build);
        if common_dir.exists() {
            scanner_build.include(&common_dir);
        }
        if let Err(e) = scanner_build.try_compile(&format!("tree_sitter_{name}_scanner")) {
            println!("cargo:warning=Failed to compile C scanner for '{}': {}", name, e);
            return false;
        }
    }

    // Step 3: Compile scanner.cc (C++ scanners) separately with same prefixing.
    let scanner_cc = src_dir.join("scanner.cc");
    if scanner_cc.exists() {
        let mut cpp_build = cc::Build::new();
        cpp_build
            .include(&src_dir)
            .file(&scanner_cc)
            .define("TREE_SITTER_HIDE_SYMBOLS", None)
            .define("scan", &*format!("tree_sitter_{name}_ext_scan"))
            .define("deserialize", &*format!("tree_sitter_{name}_ext_deserialize"))
            .define("serialize", &*format!("tree_sitter_{name}_ext_serialize"))
            .flag_if_supported("-fvisibility=hidden")
            .warnings(false)
            .cpp(true);
        apply_wasm32_sysroot(&mut cpp_build);
        apply_wasm32_optimizations(&mut cpp_build);
        if common_dir.exists() {
            cpp_build.include(&common_dir);
        }
        if let Err(e) = cpp_build.try_compile(&format!("tree_sitter_{name}_scanner_cpp")) {
            println!("cargo:warning=Failed to compile C++ scanner for '{}': {}", name, e);
            return false;
        }
    }

    true
}

fn generate_registry(
    static_langs: &[String],
    dynamic_langs: &[String],
    definitions: &BTreeMap<String, LanguageDefinition>,
    libs_dir: &Path,
    out_dir: &Path,
) {
    let path = out_dir.join("registry_generated.rs");
    let mut f = fs::File::create(&path).expect("Failed to create registry_generated.rs");

    writeln!(f, "// Auto-generated by build.rs — do not edit").unwrap();
    writeln!(
        f,
        "// This file is include!()'d into registry.rs which imports tree_sitter::Language"
    )
    .unwrap();
    writeln!(f).unwrap();

    // Resolve C symbol name: use c_symbol override if present, otherwise use language name
    let c_sym = |name: &str| -> String {
        definitions
            .get(name)
            .and_then(|d| d.c_symbol.as_deref())
            .unwrap_or(name)
            .to_string()
    };

    // Static extern declarations
    if !static_langs.is_empty() {
        for name in static_langs {
            let sym = c_sym(name);
            writeln!(f, "unsafe extern \"C\" {{").unwrap();
            writeln!(f, "    fn tree_sitter_{sym}() -> *const tree_sitter::ffi::TSLanguage;").unwrap();
            writeln!(f, "}}").unwrap();
        }
        writeln!(f).unwrap();
    }

    // Static languages table
    writeln!(
        f,
        "#[allow(unused, clippy::type_complexity)]\npub(crate) static STATIC_LANGUAGES: &[(&str, fn() -> Language)] = &["
    )
    .unwrap();
    for name in static_langs {
        let sym = c_sym(name);
        writeln!(
            f,
            "    (\"{name}\", || unsafe {{ Language::from_raw(tree_sitter_{sym}()) }}),",
        )
        .unwrap();
    }
    writeln!(f, "];").unwrap();
    writeln!(f).unwrap();

    // Dynamic languages list and libs directory
    writeln!(
        f,
        "#[allow(unused)]\npub(crate) static DYNAMIC_LANGUAGE_NAMES: &[&str] = &["
    )
    .unwrap();
    for name in dynamic_langs {
        writeln!(f, "    \"{name}\",").unwrap();
    }
    writeln!(f, "];").unwrap();
    writeln!(f).unwrap();

    // Use {:?} for proper escaping of the path string
    writeln!(
        f,
        "#[allow(unused)]\npub(crate) static LIBS_DIR: &str = {:?};",
        libs_dir.display().to_string()
    )
    .unwrap();
    writeln!(f).unwrap();

    // C symbol overrides: language name -> C symbol (only for languages where they differ)
    writeln!(
        f,
        "#[allow(unused)]\npub(crate) static C_SYMBOL_OVERRIDES: &[(&str, &str)] = &["
    )
    .unwrap();
    for name in static_langs.iter().chain(dynamic_langs.iter()) {
        if let Some(def) = definitions.get(name.as_str())
            && let Some(c_sym) = &def.c_symbol
        {
            writeln!(f, "    (\"{name}\", \"{c_sym}\"),").unwrap();
        }
    }
    writeln!(f, "];").unwrap();
}

/// Emit rerun-if-changed for specific source files in a parser directory.
fn emit_rerun_if_changed(parser_dir: &Path) {
    let src_dir = parser_dir.join("src");
    for file in &["parser.c", "scanner.c", "scanner.cc"] {
        let path = src_dir.join(file);
        if path.exists() {
            println!("cargo:rerun-if-changed={}", path.display());
        }
    }
}

/// Generate extension-to-language lookup code from language_definitions.json.
///
/// Reads extensions from the JSON definitions and generates a match expression
/// that maps file extensions to language names. Validates strictly:
/// - No duplicate extensions across languages
/// - Extensions must be ASCII, lowercase, non-empty
/// - Extensions must not contain dots, whitespace, or special characters
fn generate_extensions_lookup(definitions: &BTreeMap<String, LanguageDefinition>, out_dir: &Path) {
    let mut ext_to_lang: BTreeMap<String, String> = BTreeMap::new();

    for (lang_name, def) in definitions {
        for ext in &def.extensions {
            // Validate: non-empty
            if ext.is_empty() {
                panic!("Empty extension for language '{lang_name}' in language_definitions.json");
            }
            // Validate: ASCII only
            if !ext.is_ascii() {
                panic!("Non-ASCII extension '{ext}' for language '{lang_name}' in language_definitions.json");
            }
            // Validate: lowercase
            if ext != &ext.to_ascii_lowercase() {
                panic!("Extension '{ext}' for language '{lang_name}' must be lowercase in language_definitions.json");
            }
            // Validate: no dots
            if ext.contains('.') {
                panic!(
                    "Extension '{ext}' for language '{lang_name}' must not contain dots (use 'py' not '.py') in language_definitions.json"
                );
            }
            // Validate: no whitespace or special characters (allow only alphanumeric and underscore)
            if !ext.chars().all(|c| c.is_ascii_alphanumeric() || c == '_') {
                panic!(
                    "Extension '{ext}' for language '{lang_name}' contains invalid characters in language_definitions.json. Only alphanumeric and underscore allowed."
                );
            }
            // Validate: no duplicates
            if let Some(existing) = ext_to_lang.get(ext) {
                panic!(
                    "Duplicate extension '{ext}' in language_definitions.json: mapped to both '{existing}' and '{lang_name}'"
                );
            }
            ext_to_lang.insert(ext.clone(), lang_name.clone());
        }
    }

    // Group extensions by language name for compact match arms
    let mut lang_to_exts: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for (ext, lang) in &ext_to_lang {
        lang_to_exts.entry(lang.as_str()).or_default().push(ext.as_str());
    }

    let path = out_dir.join("extensions_generated.rs");
    let mut f = fs::File::create(&path).expect("Failed to create extensions_generated.rs");

    writeln!(
        f,
        "// Auto-generated by build.rs from sources/language_definitions.json — do not edit"
    )
    .unwrap();
    writeln!(f, "{{").unwrap();
    writeln!(f, "    let mut buf = [0u8; 32];").unwrap();
    writeln!(f, "    let ext_lower = if ext.len() <= buf.len() && ext.is_ascii() {{").unwrap();
    writeln!(f, "        for (i, b) in ext.bytes().enumerate() {{").unwrap();
    writeln!(f, "            buf[i] = b.to_ascii_lowercase();").unwrap();
    writeln!(f, "        }}").unwrap();
    writeln!(f, "        match std::str::from_utf8(&buf[..ext.len()]) {{").unwrap();
    writeln!(f, "            Ok(s) => s,").unwrap();
    writeln!(f, "            Err(_) => return None,").unwrap();
    writeln!(f, "        }}").unwrap();
    writeln!(f, "    }} else {{").unwrap();
    writeln!(f, "        return None;").unwrap();
    writeln!(f, "    }};").unwrap();
    writeln!(f).unwrap();
    writeln!(f, "    match ext_lower {{").unwrap();

    for (lang, exts) in &lang_to_exts {
        let patterns: Vec<String> = exts.iter().map(|e| format!("\"{e}\"")).collect();
        writeln!(f, "        {} => Some(\"{lang}\"),", patterns.join(" | ")).unwrap();
    }

    writeln!(f, "        _ => None,").unwrap();
    writeln!(f, "    }}").unwrap();
    writeln!(f, "}}").unwrap();

    // Generate ambiguity lookup: extension -> (assigned_language, [alternative_languages])
    let mut ambiguities: BTreeMap<String, (String, Vec<String>)> = BTreeMap::new();
    for (lang_name, def) in definitions {
        for (ext, alternatives) in &def.ambiguous {
            // Validate: the ambiguous extension must be in this language's extensions list
            if !def.extensions.contains(ext) {
                panic!("Ambiguous extension '{ext}' for language '{lang_name}' is not in its extensions list");
            }
            // Validate: alternative languages must exist in definitions
            for alt in alternatives {
                if !definitions.contains_key(alt) {
                    panic!(
                        "Ambiguous alternative '{alt}' for extension '{ext}' (language '{lang_name}') does not exist in language_definitions.json"
                    );
                }
            }
            ambiguities.insert(ext.clone(), (lang_name.clone(), alternatives.clone()));
        }
    }

    let ambig_path = out_dir.join("ambiguities_generated.rs");
    let mut af = fs::File::create(&ambig_path).expect("Failed to create ambiguities_generated.rs");

    writeln!(
        af,
        "// Auto-generated by build.rs from sources/language_definitions.json — do not edit"
    )
    .unwrap();
    writeln!(af, "match ext_lower {{").unwrap();
    for (ext, (assigned, alternatives)) in &ambiguities {
        let alt_strs: Vec<String> = alternatives.iter().map(|a| format!("\"{a}\"")).collect();
        writeln!(
            af,
            "    \"{ext}\" => Some((\"{assigned}\", &[{}])),",
            alt_strs.join(", ")
        )
        .unwrap();
    }
    writeln!(af, "    _ => None,").unwrap();
    writeln!(af, "}}").unwrap();
}

/// Generate bundled highlight/injection/locals query functions from parsers/{lang}/queries/*.scm.
///
/// Scans the parsers directory for query files and generates a Rust source file with three
/// match functions: `get_highlights_query_impl`, `get_injections_query_impl`, and
/// `get_locals_query_impl`. Only languages that actually have the relevant .scm file on
/// disk at build time are included.
fn generate_queries_registry(definitions: &BTreeMap<String, LanguageDefinition>, parsers_dir: &Path, out_dir: &Path) {
    let path = out_dir.join("queries_generated.rs");
    let mut f = fs::File::create(&path).expect("Failed to create queries_generated.rs");

    writeln!(f, "// Auto-generated by build.rs — do not edit").unwrap();
    writeln!(f).unwrap();

    // Collect languages that have each query type
    let mut highlights: Vec<String> = Vec::new();
    let mut injections: Vec<String> = Vec::new();
    let mut locals: Vec<String> = Vec::new();

    for lang in definitions.keys() {
        let queries_dir = parsers_dir.join(lang).join("queries");
        if queries_dir.join("highlights.scm").exists() {
            highlights.push(lang.clone());
            println!(
                "cargo:rerun-if-changed={}",
                queries_dir.join("highlights.scm").display()
            );
        }
        if queries_dir.join("injections.scm").exists() {
            injections.push(lang.clone());
            println!(
                "cargo:rerun-if-changed={}",
                queries_dir.join("injections.scm").display()
            );
        }
        if queries_dir.join("locals.scm").exists() {
            locals.push(lang.clone());
            println!("cargo:rerun-if-changed={}", queries_dir.join("locals.scm").display());
        }
    }

    // Helper: generate a query lookup function, or just return None if empty.
    // We read query file contents at build time and embed them as string literals
    // instead of using include_str! with relative paths, because the relative paths
    // (../../parsers/) break when the crate is packaged for cargo publish.
    let gen_query_fn = |f: &mut fs::File, name: &str, langs: &[String], query_file: &str| {
        writeln!(f, "pub(crate) fn {name}(lang: &str) -> Option<&'static str> {{").unwrap();
        if langs.is_empty() {
            writeln!(f, "    let _ = lang;").unwrap();
            writeln!(f, "    None").unwrap();
        } else {
            writeln!(f, "    match lang {{").unwrap();
            for lang in langs {
                let query_path = parsers_dir.join(lang).join("queries").join(query_file);
                let contents = fs::read_to_string(&query_path)
                    .unwrap_or_else(|e| panic!("Failed to read {}: {e}", query_path.display()));
                let escaped = contents.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n");
                writeln!(f, "        \"{lang}\" => Some(\"{escaped}\"),",).unwrap();
            }
            writeln!(f, "        _ => None,").unwrap();
            writeln!(f, "    }}").unwrap();
        }
        writeln!(f, "}}").unwrap();
        writeln!(f).unwrap();
    };

    gen_query_fn(&mut f, "get_highlights_query_impl", &highlights, "highlights.scm");
    gen_query_fn(&mut f, "get_injections_query_impl", &injections, "injections.scm");
    gen_query_fn(&mut f, "get_locals_query_impl", &locals, "locals.scm");
}

fn main() {
    println!("cargo:rerun-if-env-changed=TSLP_LANGUAGES");
    println!("cargo:rerun-if-env-changed=PROJECT_ROOT");
    println!("cargo:rerun-if-env-changed=TSLP_LINK_MODE");

    let project_root = find_project_root();

    // When installed from crates.io without lang-* features, the project root
    // (and language_definitions.json) won't exist. Generate an empty registry
    // so the crate builds — dynamic loading + download handles parsers at runtime.
    let definitions_path = project_root.join("sources/language_definitions.json");
    let definitions: BTreeMap<String, LanguageDefinition> = if definitions_path.exists() {
        println!("cargo:rerun-if-changed={}", definitions_path.display());
        let definitions_json = fs::read_to_string(&definitions_path)
            .unwrap_or_else(|e| panic!("Failed to read {}: {e}", definitions_path.display()));
        serde_json::from_str(&definitions_json).expect("Failed to parse language_definitions.json")
    } else {
        // No definitions available (e.g. crates.io install) — empty set
        BTreeMap::new()
    };
    let parsers_dir = project_root.join("parsers");

    let selected = selected_languages(&definitions);

    // Determine link mode: "dynamic" (default), "static", or "both"
    // Force static mode for wasm32 targets (no shared library support)
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();
    let link_mode = if target_arch == "wasm32" {
        "static".to_string()
    } else {
        env::var("TSLP_LINK_MODE").unwrap_or_else(|_| "dynamic".to_string())
    };

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let libs_dir = out_dir.join("libs");
    fs::create_dir_all(&libs_dir).expect("Failed to create libs directory");

    let mut static_compiled = Vec::new();
    let mut dynamic_compiled = Vec::new();

    for name in &selected {
        let parser_dir = parsers_dir.join(name);
        if !parser_dir.join("src/parser.c").exists() {
            println!(
                "cargo:warning=Parser sources not found for '{}' at {}",
                name,
                parser_dir.display()
            );
            continue;
        }

        emit_rerun_if_changed(&parser_dir);

        match link_mode.as_str() {
            "static" => {
                if compile_parser_static(name, &parser_dir) {
                    static_compiled.push(name.clone());
                }
            }
            "dynamic" => {
                if compile_parser_dynamic(name, &parser_dir, &libs_dir) {
                    dynamic_compiled.push(name.clone());
                }
            }
            "both" => {
                if compile_parser_static(name, &parser_dir) {
                    static_compiled.push(name.clone());
                }
                if compile_parser_dynamic(name, &parser_dir, &libs_dir) {
                    dynamic_compiled.push(name.clone());
                }
            }
            _ => {
                println!(
                    "cargo:warning=Unknown TSLP_LINK_MODE '{}', defaulting to dynamic",
                    link_mode
                );
                if compile_parser_dynamic(name, &parser_dir, &libs_dir) {
                    dynamic_compiled.push(name.clone());
                }
            }
        }
    }

    generate_registry(&static_compiled, &dynamic_compiled, &definitions, &libs_dir, &out_dir);
    generate_extensions_lookup(&definitions, &out_dir);
    generate_queries_registry(&definitions, &parsers_dir, &out_dir);
}
