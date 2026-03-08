# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [1.0.0-rc.1] - 2026-XX-XX

Complete rewrite from Python to Rust with polyglot language bindings.

### Added

- Rust core library (`ts-pack-core`) with `LanguageRegistry` for thread-safe grammar access
- C-FFI layer (`ts-pack-ffi`) with cbindgen-generated headers and panic shields
- Python bindings via PyO3/maturin (`ts-pack-python`) with PyCapsule support
- Node.js bindings via NAPI-RS (`ts-pack-node`) with TypeScript definitions
- Go bindings via cgo (`ts-pack-go`) with platform-specific build directives
- Java bindings via Panama FFM (`ts-pack-java`) targeting JDK 22+
- Elixir bindings via Rustler NIF (`ts-pack-elixir`) with ExUnit tests
- CLI tool (`ts-pack-cli`) for grammar management (init, list, add, remove, info, build)
- E2E test generator with 7 language backends (Rust, Python, TypeScript, Go, Java, Elixir, C)
- 21 test fixtures across 4 categories (smoke, parsing, error handling, registry)
- Dynamic linking mode (`TSLP_LINK_MODE=dynamic`) for per-parser shared libraries
- Feature-gated language selection via `TSLP_LANGUAGES` env var or Cargo features
- Language group features: `web`, `systems`, `scripting`, `data`, `jvm`, `functional`
- Tree-sitter 0.26 support with `Language::into_raw()` / `Language::from_raw()`
- Domain-split CI workflows (ci-validate, ci-rust, ci-python, ci-node, ci-go, ci-java, ci-elixir, ci-c)
- Multi-registry publish workflow (crates.io, PyPI, npm, GitHub Releases for Go FFI)
- 168 language grammars supported

### Changed

- Architecture: Python-only package → Rust core with polyglot bindings
- Parser compilation: pure Python with tree-sitter CLI → Rust `build.rs` with `cc` crate
- Language registry: dictionary-based → typed `LanguageRegistry` with thread-safe `LazyLock` access
- Error handling: Python exceptions → Rust `Result<T, E>` with cross-language error conversion

### Removed

- Python-only implementation (setup.py, MANIFEST.in, tree_sitter_language_pack/)
- Direct tree-sitter Python dependency for parsing (now via native bindings)
- Cython-based build pipeline

---

## Pre-1.0 Releases (Python-only)

### [0.12.0]

#### Added

- tree-sitter-cobol grammar support

#### Fixed

- MSVC build compatibility for cobol grammar
- Alpine Linux (musl) wheel platform tag support (PEP 656)
- Wheel file discovery in CI test action

### [0.11.0]

#### Added

- tree-sitter-bsl (1C:Enterprise) grammar support

#### Changed

- Updated all dependencies and relocked

### [0.10.0]

#### Added

- tree-sitter 0.25 support

#### Changed

- Dropped Python 3.9 support
- Adopted prek pre-commit workflow
- CI: cancel superseded workflow runs

### [0.9.1]

#### Added

- WASM (wast & wat) grammar support
- F# and F# signature grammar support

### [0.9.0]

#### Added

- tree-sitter-nim grammar support
- tree-sitter-ini grammar support
- Swift grammar update (trailing comma support)

### [0.8.0]

#### Fixed

- sdist build issues resolved

### [0.7.4]

#### Added

- GraphQL grammar support
- Kotlin grammar support (SAM conversions)
- Netlinx grammar support

### [0.7.3]

#### Changed

- Swift grammar update (macros + copyable)

### [0.7.2]

#### Added

- Apex grammar support

#### Fixed

- MSYS2 GCC build issues

### [0.7.1]

#### Added

- OCaml and OCaml Interface grammar support
- Markdown inline parser support

#### Fixed

- Pinned elm and rust grammar versions
- Pinned tree-sitter-tcl to known-good revision

### [0.6.1]

#### Added

- ARM64 Linux CI builds

#### Fixed

- Build issue resolved

### [0.6.0]

#### Fixed

- Windows DLL loading compatibility issues

### [0.5.0]

#### Fixed

- Windows compatibility and encoding issues for non-English locales

### [0.4.0]

#### Added

- PyCapsule-based language loading
- Protocol Buffers (proto) grammar support
- SPARQL grammar support

### [0.3.0]

#### Changed

- Updated generation setup and build matrix
- Removed magik and swift grammars (temporarily)

### [0.2.0]

#### Changed

- Version bump with dependency updates

### [0.1.2]

#### Fixed

- Added MANIFEST.in for sdist packaging

### [0.1.1]

#### Fixed

- Missing parsers in package data

### [0.1.0]

#### Added

- Initial release with 100+ tree-sitter language grammars
- Python package with pre-compiled parsers
- Multi-platform wheel builds (Linux, macOS, Windows)
