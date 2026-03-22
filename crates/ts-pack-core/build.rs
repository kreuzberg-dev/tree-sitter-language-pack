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

fn generate_registry(static_langs: &[String], dynamic_langs: &[String], libs_dir: &Path, out_dir: &Path) {
    let path = out_dir.join("registry_generated.rs");
    let mut f = fs::File::create(&path).expect("Failed to create registry_generated.rs");

    writeln!(f, "// Auto-generated by build.rs — do not edit").unwrap();
    writeln!(
        f,
        "// This file is include!()'d into registry.rs which imports tree_sitter::Language"
    )
    .unwrap();
    writeln!(f).unwrap();

    // Static extern declarations
    if !static_langs.is_empty() {
        for name in static_langs {
            writeln!(f, "unsafe extern \"C\" {{").unwrap();
            writeln!(f, "    fn tree_sitter_{name}() -> *const tree_sitter::ffi::TSLanguage;").unwrap();
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
        writeln!(
            f,
            "    (\"{name}\", || unsafe {{ Language::from_raw(tree_sitter_{name}()) }}),",
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

    generate_registry(&static_compiled, &dynamic_compiled, &libs_dir, &out_dir);
}
