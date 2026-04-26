# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Fixed

- PHP: fix broken `crates/ts-pack-php/README.md` links in root `README.md` — path moved to `packages/php/README.md` after alef migration (#106)
- PHP: fix `.task/php.yml` `build`, `build:dev`, and `clean` tasks pointing to removed `crates/ts-pack-php/` — corrected to `crates/ts-pack-core-php/` (#106)
- PHP: align `packages/php/composer.json` and `packages/php/README.md` package name to canonical Packagist vendor slug (`kreuzberg/` not `kreuzberg-dev/`) (#106)
- PHP: document `mlocati/php-extension-installer` prerequisite in install docs and correct minimum PHP version to 8.4+ (#106)
- Go: regenerate stale `binding.go` with current alef generator

## [1.7.0] - 2026-04-22

### Added

- Migrate to [alef](https://github.com/kreuzberg-dev/alef) polyglot binding generator — all language bindings (Python, TypeScript, Ruby, Go, Java, C#, Elixir, PHP, WASM) are now generated from a single `alef.toml` configuration
- `Default`, `Hash`, `PartialEq`, `Eq` derives on all public types
- 18 new e2e test fixtures closing testing gaps across all binding languages
- Consolidated CI: 12 language-specific workflows merged into a single `ci.yaml`
- Registry-mode e2e test apps under `test_apps/` (generated via `alef e2e generate --registry`)

### Changed

- Public API locked down with `pub(crate)` — only functions and types that were in the pre-alef Python bindings are exported; internal modules (`json_utils`, `intel` submodules, `config`, `definitions`) are no longer public
- Workspace lints applied to all binding crates (`clippy::all = "deny"`, `unsafe_code = "deny"`)
- `test_apps/` moved from `tests/test_apps/` to project root

### Fixed

- `available_languages()`, `has_language()`, and `language_count()` now register the download cache directory before querying the registry — fixes empty results when using the `download` feature (#90)
- `process()` auto-downloads missing parsers instead of returning `LanguageNotFound` (#94)
- C# task references updated from `.sln` to `.csproj`
- Maven version plugin pinned to exclude alpha/beta/RC versions
- Docker CI: `uv run` changed to `uv run --no-project` to avoid triggering root pyproject.toml build
- Ruby CI: removed stale `working-directory` that pointed to wrong path

## [1.6.3] - 2026-04-20

### Fixed

- Go: fix FFI build defaults — add `TSLP_LINK_MODE` and `TSLP_LANGUAGES` env vars to Go task (#102)
- Go: fix CGO `LDFLAGS` paths — point to workspace `target/release/` instead of crate-local path (#102)
- Go: remove duplicate forward declarations from `ffi.go` (already in `ts_pack.h`) (#102)
- Go: fix README examples — proper error handling, correct API signatures (`Init`, `Download`) (#102)
- FFI: add extra libs dir from `cache_dir()` to registry on creation (#102)
- Docs: fix textlint pre-commit hook — add `additional_dependencies` for all textlint plugins (#102)

## [1.6.2] - 2026-04-18

### Fixed

- Compile bundled grammars with `-fno-strict-aliasing` to prevent undefined behavior (#100)

### Changed

- Update dependencies across lockfiles
- Regenerate READMEs for 1.6.1 version bump (#101)

## [1.6.1] - 2026-04-17

### Fixed

- Go: move package root from `packages/go/v1/` to `packages/go/` so the Go module proxy can resolve `go.mod` at the correct path — `go get github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go` now works (#97)
- Go: fix CGO `SRCDIR`-relative include/lib paths (one fewer `../` after directory restructure)
- Remove `features = ["all"]` from e2e Rust test `Cargo.toml` — use `download` feature for runtime parser fetching
- Remove 305 `lang-*` features to unblock crates.io publish (300 feature limit)
- Regenerate READMEs for v1.6.0, fix Windows query cache test flake
- Bump `rustls-webpki` to patch RUSTSEC-2026-0098 and RUSTSEC-2026-0099 (#99)
- Fix MIME type inference in core build by embedding `language_definitions.json` in crate

### Changed

- Update dependencies across Python, Node.js, PHP, and Rust lockfiles
- Replace feature group docs with `download`/`TSLP_LANGUAGES` documentation in READMEs

## [1.6.0] - 2026-04-14

### Added

- Thread-local parser cache in `parse_string()` — avoids re-creating parsers on repeated calls for the same language
- Two-level compiled query cache (thread-local + global) in `run_query()` — avoids recompiling tree-sitter queries
- `parse_with_language()` internal API for callers that already have a `Language` object
- Pre-computed capture names in `CompiledExtraction` — avoids rebuilding on every extraction call
- Go `type_spec` declarations extracted as symbols with correct `SymbolKind` (struct, interface, type)
- Dedicated "Download Parsers" section in quickstart docs covering CLI, programmatic APIs, groups, Docker/CI, and config files
- Tests for parser cache reuse, query cache sharing across threads, cursor byte-range isolation, and capture name correctness

### Fixed

- `compiled_query()` now propagates `Error::LockPoisoned` instead of silently ignoring poisoned RwLock
- `QueryCursor` byte-range no longer leaks between patterns when reusing the cursor in `extract_from_tree()`
- Replaced `std::collections::HashMap` with `ahash::AHashMap` in parser cache for consistency
- Redundant `get_language()` call removed from `parse_string()` hot path — only called on cache miss

### Changed

- `CompiledExtraction::extract()` and `intel::parse_source()` now use the thread-local parser cache
- `QueryCursor` reused across patterns within a single `extract_from_tree()` call
- Unnecessary `String` allocation removed from `node_types.contains()` check in chunking

### Removed

- All 305 `lang-*` Cargo features and group features (`all`, `web`, `systems`, `scripting`, `data`, `jvm`, `functional`, `wasm`) — language selection is now via `TSLP_LANGUAGES` env var at build time; the `download` feature (default) fetches parsers at runtime

## [1.5.0] - 2026-04-08

### Added

- 57 new permissively-licensed grammars — 305 languages total
  - abl, c3, cel, cfml, chuck, cst, dhall, elvish, gap, gdshader, glimmer, gnuplot, gotmpl, gowork, gpg, hjson, hocon, hoon, htmldjango, jai, javadoc, json5, kcl, mlir, nasm, norg_meta, ocamllex, openscad, phpdoc, poe_filter, prql, rasi, razor, rbs, roc, rtf, slang, smalltalk, sml, snakemake, souffle, sourcepawn, sql_bigquery, stan, superhtml, sway, systemverilog, tact, tera, typespec, typoscript, vhs, vrl, wgsl_bevy, x86asm, ziggy, ziggy_schema
- CI license validation job in `ci-validate.yaml` — blocks PRs that introduce non-permissive (GPL/AGPL/LGPL/MPL) grammars

### Fixed

- `less` grammar: regenerated parser from ABI 11 to ABI 14 (was incompatible with tree-sitter 0.26)
- `corn` smoke fixture: replaced invalid `"x"` snippet with valid corn syntax

## [1.4.1] - 2026-03-31

### Fixed

- Include `language_definitions.json` in the published crate so `build.rs` can find extension mappings, ambiguity data, and C symbol overrides when installed from crates.io

### Changed

- Updated dependencies across all language ecosystems

## [1.4.0] - 2026-03-29

### Fixed

- Expose `detect_language` in Python public API (#85)
- PHP extension name corrected to `ts-pack-php` (hyphens)

### Changed

- All language snippet READMEs and documentation corrected
- Removed automated grammar updates workflow

## [1.3.3] - 2026-03-27

### Fixed

- `C_SYMBOL_OVERRIDES` table now includes ALL languages from `language_definitions.json`, not just compiled ones — fixes download and loading of `csharp`, `vb`, `embeddedtemplate`, `nushell` from PyPI/npm/RubyGems packages
- `downloaded_languages()` returns canonical names (`csharp`) instead of c_symbol names (`c_sharp`)
- Elixir NIF publish: upload both hyphen and underscore artifact names so RustlerPrecompiled can find them
- Elixir NIF 2.17 packaging: fix stale variable names from dual-name refactor
- Ruby comprehensive test: remove `JSON.parse` on native Hash return from `process()`
- Go comprehensive test: access flat `ProcessResult` fields directly (no `metadata` wrapper)
- Homebrew bottle and PHP PIE packages now included in release artifacts

### Changed

- Dependency updates across all language ecosystems
- `rustler_precompiled` updated to 0.9.0 (Elixir)

## [1.3.2] - 2026-03-26

### Fixed

- Dynamic parser loading for languages with `c_symbol` overrides (`csharp`, `vb`, `embeddedtemplate`, `nushell`) — build was naming libraries with the raw name but runtime loader expected the `c_symbol` name (#80)
- Go E2E generator: unused `tspack` import in non-process test files
- Elixir: add missing `extract/2` and `validate_extraction/1` NIF declarations
- PHP E2E generator: use double-quoted strings for source code so `\n` is interpreted correctly
- Nim grammar: switch from abandoned `paranim/tree-sitter-nim` (ABI v11) to `aMOPel/tree-sitter-nim` (MIT, ABI v14)

### Added

- Smoke test fixtures for all `c_symbol` override languages (csharp, vb, embeddedtemplate, nushell)
- Dynamic-linking CI step in `ci-all-grammars.yaml` to catch `c_symbol` naming mismatches

## [1.3.1] - 2026-03-26

### Fixed

- Ruby binding: `process()`, `extract()`, `validate_extraction()` now return native Ruby Hash instead of raw JSON string
- WASM binding: output keys now use camelCase (matching Node.js binding convention), input config accepts both camelCase and snake_case
- Go E2E generator: use typed `*ProcessResult` struct fields instead of invalid `json.Unmarshal` on non-string return
- Elixir CI: stage NIF with both hyphenated and underscored filenames to satisfy Rustler force-build check and `load_from` loader

## [1.3.0] - 2026-03-26

### Added

- Extraction query API: run user-defined tree-sitter queries and get structured results
  - `extract_patterns()` / `extract()` across Python, Node.js, Rust, Ruby, Elixir, PHP, WASM, C FFI
  - `validate_extraction()` for config validation without execution
  - `CompiledExtraction` for pre-compiled query reuse (Rust)
  - `ProcessConfig.extractions` for combining custom queries with standard analysis
  - Types: ExtractionConfig, ExtractionPattern, CaptureOutput, CaptureResult, MatchResult, PatternResult, ExtractionResult
- Criterion benchmarks: 9 groups, 23 benchmarks across Python, TypeScript, Rust, Go
- Extraction queries guide and documentation across all API references

### Fixed

- E2E generator: `process_imports_contains_source` assertion uses contains instead of equality
- WASM: language list matches actual compiled features (30 languages)
- WASM: add missing `detectLanguageFromPath` and `detectLanguageFromExtension` exports
- PHP generator: null array handling in `process()` result assertions
- Elixir: RustlerPrecompiled `crate` field resolution with `load_from` override
- Predicate evaluation: remove redundant re-evaluation (tree-sitter 0.26 handles internally)
- Documentation: stale version numbers, incomplete API references, incorrect function signatures
- Java version requirement standardized to JDK 25+

## [1.2.1] - 2026-03-25

### Fixed

- Nushell grammar `c_symbol` override — linker error `undefined symbol: tree_sitter_nushell`
- E2E generator calling `.as_deref()` on `String` type (compile error on CI)
- WASM build: gate `c_symbol_for` behind `dynamic-loading`/`download` features (dead code warning)
- Elixir publish: align RustlerPrecompiled `crate:` field with Cargo `[lib]` name (underscores, not hyphens)
- Elixir publish: add `--cfg` flag patch to publish workflow for Rustler 0.37.3 compatibility
- Python `without_gil()`: add `catch_unwind` to ensure GIL is reacquired on panic
- Text splitter: prevent zero-width chunks in pathological UTF-8 edge case
- Comment kind detection: handle `//!`, `/*!`, and `doc_comment` node types
- Import detection: restrict fallback to explicitly supported languages only
- Export detection: use field-based AST matching instead of fragile `text.contains()`

### Changed

- Registry: `Arc<Vec<PathBuf>>` for extra lib dirs (avoids Vec clone per language lookup)
- Registry: `AHashSet<&str>` in `available_languages()` (avoids 248+ String allocations)
- `NodeInfo.kind` uses `Cow::Borrowed` (zero-copy from tree-sitter's `&'static str`)
- Python: `with_tree()`/`try_with_tree()` helpers replace 9 duplicate lock patterns
- Python: `without_gil()` helper replaces 5 duplicate GIL release patterns
- Core: `extension_ambiguity_json()` helper replaces duplicated JSON serialization in 4 bindings
- Chunking: `MetadataCollector` struct reduces function from 11 to 7 parameters
- FFI: 25 SAFETY comments added to unsafe blocks
- Docs: rewrite all 12 API references to match actual binding source code
- Docs: add JSON-LD structured data and Open Graph metadata for crawlers

## [1.2.0] - 2026-03-25

### Added

- 49 new permissively-licensed grammars — 248 languages total
  - angular, bass, blade, brightscript, circom, cooklang, corn, crystal, cue, cylc, desktop, djot, earthfile, ebnf, editorconfig, eds, eex, elsa, enforce, facility, faust, fidl, foam, forth, git_config, git_rebase, godot_resource, http, hurl, just, ledger, less, liquid, mojo, move, nickel, nginx, norg, nushell, promql, pug, ql, robot, teal, templ, tmux, todotxt, turtle, vimdoc, wolfram
- Grammar updater automation (`scripts/check_grammar_updates.py`) with weekly CI workflow
- Generated supported languages table (`docs/supported-languages.md`) integrated into docs CI
- Node.js NAPI exports: `detectLanguageFromExtension`, `detectLanguageFromPath`, `getHighlightsQuery`, `extensionAmbiguity`
- E2E `process` test category with `process()` API coverage across all 11 language bindings

### Fixed

- Download/load filename mismatch for languages with c_symbol overrides (csharp, embeddedtemplate, vb) — fixes [#80](https://github.com/kreuzberg-dev/tree-sitter-language-pack/issues/80)
- E2E fixture system: merged stale `intel/` and `metadata/` directories into unified `process/` category
- TypeScript and WASM e2e generators now use camelCase for metrics keys
- Docker CI grammar fixture updated to include all languages
- Elixir publish workflow: checksum file verification, increased retry timeout
- Missing Node.js `index.js` exports for detection and query functions

### Changed

- Renamed e2e fixture assertions from `intel_*`/`meta_*` to `process_*`
- All documentation and package descriptions updated to reflect 248 languages

## [1.1.4] - 2026-03-24

### Added

- New language: `al` (AL / Business Central) — 198 languages total
- Grammar license linter (`scripts/lint_grammar_licenses.py`, `task lint:licenses`) verifies all grammars use permissive licenses
- Permissive license policy documented in CONTRIBUTING.md, docs, and README

### Fixed

- Replace `nim` grammar (alaviss, MPL-2.0 copyleft) with paranim/tree-sitter-nim (MIT)
- Replace `prolog` grammar (codeberg foxy, AGPL-3.0 copyleft) with Rukiza/tree-sitter-prolog (ISC)
- Docs: align mkdocs config with kreuzberg branding; mermaid diagrams now render (fixes [#81](https://github.com/kreuzberg-dev/tree-sitter-language-pack/issues/81))

## [1.1.3] - 2026-03-24

### Fixed

- Dynamic loader: resolve `c_symbol` overrides for csharp, embeddedtemplate, and vb so `get_language()` works for dynamically loaded grammars (fixes [#80](https://github.com/kreuzberg-dev/tree-sitter-language-pack/issues/80))
- E2E generator: enable all ProcessConfig features (structure, imports, exports, comments, docstrings, symbols, diagnostics) for intel tests so diagnostics assertions pass

### Added

- 23 new smoke test fixtures for languages missing coverage: asciidoc, awk, batch, caddy, cedar, cedarschema, csharp, devicetree, diff, dot, embeddedtemplate, idris, jinja2, jq, lean, pkl, postscript, prolog, rescript, ssh_config, textproto, tlaplus, vb, wit, zsh
- CI workflow (`ci-all-grammars.yaml`) that tests all 197 grammars end-to-end, preventing regressions like #80
- `rust:e2e:all-grammars` task for running the full grammar suite locally

## [1.1.2] - 2026-03-23

### Fixed

- Elixir NIF: fix Rustler crate name mismatch (`ts_pack_elixir` → `ts-pack-elixir`) causing compilation failure
- Rust crate publish: embed query file contents at build time instead of using `include_str!` with relative paths that break in the cargo package tarball

## [1.1.1] - 2026-03-23

### Fixed

- WASM build: ahash uses compile-time-rng instead of runtime-rng (avoids getrandom on wasm32)
- Docker/static build: add `c_symbol` override for grammars with non-standard C symbol names (csharp, vb, embeddedtemplate)
- Unused imports when `dynamic-loading` feature disabled (WASM builds)
- Python sdist: `.pyi` and `py.typed` now included in both wheel and sdist
- C# build: add missing `ExtensionAmbiguityResult` model class
- Set `generate: true` for csharp, vb, embeddedtemplate grammars

### Changed

- Switch from `std::HashMap`/`HashSet` to `ahash::AHashMap`/`AHashSet` for faster hashing in registry

## [1.1.0] - 2026-03-23

### Added

- 20 new languages from arborium: asciidoc, awk, caddy, cedar, cedarschema, devicetree, dot, idris, jinja2, jq, lean, postscript, prolog, rescript, ssh_config, textproto, tlaplus, vb, wasm-interface-types, zsh (197 total)
- Centralized extension-to-language mapping: `sources/language_definitions.json` is the single source of truth for 239 file extensions across 197 languages
- Build-time code generation: `build.rs` generates extension lookup with strict validation (panics on duplicates, non-ASCII, uppercase, dots)
- `detect_language_from_content(content)`: shebang-based language detection (`#!/usr/bin/env python3` → "python")
- `extension_ambiguity(ext)`: query whether a file extension is ambiguous (e.g. `.m` → objc with matlab alternative)
- Highlight query bundling: `get_highlights_query(lang)`, `get_injections_query(lang)`, `get_locals_query(lang)` — embed .scm queries at build time
- `ambiguous` field in `language_definitions.json` for declaring known extension ambiguities
- E2E test fixtures and generators for detect-language, ambiguity, and highlights across all 11 language targets
- New APIs exposed in all bindings: Python, Node.js, Ruby, WASM, Elixir, PHP, C FFI, Go, C#

### Changed

- `LanguageRegistry` uses `Arc<RwLock<Vec<PathBuf>>>` for interior mutability — no more global `RwLock` wrapper, eliminates lock poisoning risk
- `ProcessConfig.language`: `String` → `Cow<'static, str>` (zero allocation for string literals)
- `NodeInfo.kind`, `QueryMatch.captures`: `String` → `Cow<'static, str>`
- `available_languages()` uses `HashSet` for O(1) dedup instead of O(n) Vec contains
- Chunking line counting uses precomputed newline table with binary search (O(log n) per chunk vs O(n))
- Added `memchr` dependency for fast byte scanning in text splitter and chunking
- Extension/ambiguity lookups generated from JSON at build time
- `clone_vendors.py` now copies `queries/` directories alongside `src/`

### Fixed

- Strong types in all binding stubs: Python `.pyi` (TypedDicts), TypeScript `.d.ts` (interfaces), Ruby `.rbs` (record types), C# `Models.cs` (string enums replace `object`)
- Pre-existing registry test failures from global `RwLock` poisoning — test helpers now use local `LanguageRegistry::new()`
- Removed ambiguous `.os` (bsl) and `.cls` (apex/LaTeX conflict) extensions

## [1.0.0] - 2026-03-21

### Changed

- Docker: separated publish-docker workflow from main publish (180-minute timeout for multiplatform builds)
- Docker: publish-docker now triggers on `release` events and includes full smoke tests before push
- Test apps: all bindings now download languages before running tests (Ruby, Go, Elixir)
- Test apps: Rust test app adds parse_string validation tests
- Test apps: CLI smoke test adds chunking test
- Test apps: added Homebrew smoke test suite

### Fixed

- npm publish auth, Elixir NIF build, Ruby CI, WASM timeout (rc.17)
- CI: Elixir checksum, version sync improvements (rc.17)
- Rust test app: VERSION const updated from rc.11 to match actual dependency

## [1.0.0-rc.17] - 2026-03-21

### Fixed

- npm: publish authentication and registry configuration
- Elixir: NIF binary build and checksum generation
- Ruby: CI workflow fixes
- WASM: test timeout increased

## [1.0.0-rc.16] - 2026-03-20

### Fixed

- Elixir: RustlerPrecompiled `Mix.Project.config()[:version]` returns nil at compile time — use `@version` module attribute instead
- Elixir: CI compilation fails with `cargo metadata` error — add `skip_compilation?` when NIF is pre-staged
- Elixir: version sync now covers `tree_sitter_language_pack.ex` module attribute
- WASM: bundle all 165+ parsers (changed feature from `web` to `all`)
- WASM: Node.js test loading — read `.wasm` binary from disk instead of `fetch()`
- Java test app: align with binding API — `process()` returns JSON string, static methods for `download()`/`init()`
- Go test app: add to `go.work` for local module resolution
- C# test app: update target framework from `net8.0` to `net10.0`

## [1.0.0-rc.15] - 2026-03-20

### Fixed

- CI: CLI built with 0 static languages — added `TSLP_LANGUAGES` and `TSLP_LINK_MODE` to build step
- CI: CLI grammar test now filters to statically compiled subset via `TS_PACK_LANGUAGES` env var
- CI: Elixir `mix compile` tried downloading precompiled NIFs (404) — added `TSLP_BUILD=1` to force local build
- CI: Node.js lockfile mismatch — removed hardcoded `optionalDependencies` from ts-pack-node package.json (NAPI-RS adds them during publish)
- CI: Java E2E pom.xml updated (maven-compiler 3.15.0, surefire 3.5.5, JUnit 6.1.0-M1, gson 2.13.2)

### Changed

- License: unified to MIT only across all ecosystems (removed Apache-2.0 dual license)
- Task update commands aligned with kreuzberg across all bindings (Ruby, Node, Elixir, C#, Java, Go, PHP, Rust, Python, WASM)
- Maven plugins updated: compiler 3.15.0, surefire 3.5.5, source 3.4.0, javadoc 3.12.0, gpg 3.2.8, central-publishing 0.10.0
- Dependabot: added ignore rules for GitHub Actions artifact actions and internal crates
- Dependencies updated across all ecosystems via `task update`

## [1.0.0-rc.14] - 2026-03-20

### Fixed

- CI failures from rc.13 release
- Remove darwin-x64 target (not available in CI)
- Add linux-arm64-gnu target

## [1.0.0-rc.13] - 2026-03-19

### Fixed

- Docs: mermaid diagrams now render (fixed broken code fence closings in architecture.md)
- Docs: Elixir icon renders in Supported Ecosystems table (`:simple-elixir:`)
- Docs: mkdocs.yaml aligned with kreuzberg (custom palette, toc.integrate, repo icons)
- Publish: Ruby platform-specific native gems (build-native-gem.rb)
- Publish: PHP extension binaries uploaded to GitHub release
- Publish: Elixir uses rustler_precompiled for binary NIF distribution
- Publish: workflow triggers on release event (not just workflow_dispatch)
- Publish: npm tag computed in prepare job (rc→next, stable→latest)

## [1.0.0-rc.12] - 2026-03-19

### Added

- Config files aligned with kreuzberg: biome.json, rust-toolchain.toml, deny.toml, .golangci.yml, .clang-format, .taplo.toml, .shellcheckrc, tsconfig.json, .npmrc, go.work
- CI CLI workflow (`ci-cli.yaml`) with clone-vendors, build, smoke tests, grammar tests
- Vendor scripts for PHP and Ruby (`scripts/ci/php/vendor-core.py`, `scripts/ci/ruby/vendor-core.py`)
- Docker + C FFI badges in all READMEs
- `composer.lock` for reproducible PHP builds
- `tsconfig.json` for ts-pack-node, ts-pack-wasm, and tests/test_apps/node crates
- 8 missing Elixir NIF stubs: init, configure, download, download_all, manifest_languages, downloaded_languages, clean_cache, cache_dir

### Fixed

- CI: `rustfmt` and `clippy` added to `rust-toolchain.toml` components (fixes 6 CI workflows)
- CI: Go E2E tests — added `e2e/go` to `go.work` for module resolution
- CI: C# E2E tests — `setup-dotnet@v4` → `@v5`, e2e-generator C# template `net9.0` → `net10.0`
- CI: WASM E2E tests — missing tsconfig.json files caused vitest ENOENT
- CI: CLI smoke tests — added clone-vendors job so parsers are available
- Publish: npm tag computed once in prepare job (kreuzberg pattern: rc/alpha/beta → `next`, stable → `latest`)
- README badges: reordered to match kreuzberg, license badge `.svg`, docs link to kreuzberg.dev

### Removed

- Dead code: `sources/language_extension.c`, `scripts/publish/go/tag-and-push-go-module.sh`, `tools/e2e-generator/src/generators/mod.rs.bak`

## [1.0.0-rc.11] - 2026-03-19

### Added

- Multi-arch Docker build (linux/amd64 + linux/arm64)
- Full 19-test suite aligned across all 12 test apps (Python, Node, Ruby, Go, Java, C#, Elixir, PHP, WASM, C, Docker, Rust)
- Docker test app

### Fixed

- Node test: remove redundant `init()` call in download API tests
- WASM serde fix

## [1.0.0-rc.10] - 2026-03-19

### Added

- `json_utils` module with `snake_to_camel` / `camel_to_snake` conversion
- Node.js `process()` accepts both camelCase and snake_case config keys
- Expanded test suites: Ruby, Go, Elixir, Java, C#, PHP, WASM, C all at 19 tests

### Fixed

- Homebrew publish: git credentials for tap push
- `json_utils` gated behind `serde` feature flag
- `init()` accepts optional config, `process()` registers cache for download feature
- `cargo fmt` formatting

## [1.0.0-rc.9] - 2026-03-19

### Added

- Full documentation site with MkDocs Material (29 pages), GitHub Pages deploy
- 173-language grammar test fixtures
- Docker image build/publish to GHCR, ci-docker workflow
- `snippet-runner` tool with 33 code snippets for 11 languages
- `llms.txt` for AI assistant context
- All-grammars test suite

### Fixed

- Static linking symbol collisions: prefix scanner functions per language
- Enable grammar generation for batch/diff/pkl
- Homebrew upload checkout, RubyGems action, Docker clone_vendors
- Move Cargo profiles to workspace root
- Docker needs parsers from clone-vendors
- ci-docker needs tree-sitter-cli for grammar generation
- test_apps call `download()` before testing

### Removed

- Musl cross-compile from publish workflow

## [1.0.0-rc.8] - 2026-03-18

### Added

- Download/configure API across all bindings (Python, Node.js, Ruby, Go, Java, C#, Elixir, PHP, WASM, C FFI)
  - `init(config)` — configure + pre-download languages
  - `download(languages)` — download specific parsers
  - `download_all()` — download all 170+ parsers
  - `configure(config)` — set cache directory
  - `manifest_languages()` — list all available from remote manifest
  - `downloaded_languages()` — list locally cached parsers
  - `clean_cache()` — remove cached parsers
  - `cache_dir()` — get effective cache directory
- Auto-download in `get_language()` — parsers download on first use
- `PackConfig` struct with TOML file loading and directory discovery
- CLI redesign: `download`, `clean`, `list`, `parse`, `process`, `cache-dir`, `init`, `completions`
- Homebrew publishing pipeline (bottles, formula update via `kreuzberg-dev/homebrew-tap`)
- NAPI-RS multi-platform npm distribution (5 platform packages)
- Test apps for published package validation across 12 ecosystems
- E2E test fixtures for download API surface

### Fixed

- Memory leaks in Go, Java, C# bindings (individual strings in FFI arrays not freed)
- Python GIL not released during download/network I/O operations
- Elixir NIFs not scheduled as DirtyIo for network operations
- Java `parseString`/`process` passing `source.length()` instead of UTF-8 byte count
- Rust core `AtomicBool` ordering: `Relaxed` → `Acquire` in `ensure_cache_registered`
- C FFI `cache_dir` return type `*const c_char` → `*mut c_char`
- Elixir error atom: `parse_error` → `download_error` for download failures
- Platform detection: macOS aarch64 → `arm64` in `DownloadManager`
- Node.js npm publishing: multi-platform packages via `napi artifacts`
- Parser binary builds: `TSLP_LANGUAGES` set, correct output path
- `parsers.json` manifest generated and uploaded to GitHub releases
- `build.rs` graceful fallback for crates.io installs
- Maven GPG signing enabled in publish profile
- Ruby trusted publishing (gem name with underscores)

### Changed

- READMEs overhauled: correct badges (Homebrew, docs), download API docs, language-specific naming
- CLI binary name: `ts-pack`
- dotnet target: 9.0 → 10.0
- Ruby minimum: 3.2 → 3.4
- Go minimum: 1.22 → 1.26
- Smoke tests removed from publish workflow (replaced by test_apps)

## [1.0.0-rc.7] - 2026-03-17

### Fixed

- NAPI-RS multi-platform npm distribution (5 platform packages)
- Platform detection: macOS aarch64 → `arm64` in `DownloadManager`
- Language discovery: `available_languages()` scans download cache directories
- Python test_app uses `ProcessConfig` instead of raw dicts

## [1.0.0-rc.6] - 2026-03-17

### Fixed

- Platform detection: macOS aarch64 → `arm64` for parser downloads
- Language discovery in extra library directories
- Smoke tests removed from publish workflow

## [1.0.0-rc.5] - 2026-03-16

### Fixed

- Parser binary upload: newline-separated artifact patterns
- Upload artifacts script: empty array initialization, comma-separated pattern support
- Smoke test jobs: added checkout steps

## [1.0.0-rc.4] - 2026-03-16

### Fixed

- Windows parser build: `setup-python` action added
- Maven Central: GPG signing enabled in publish profile (`gpg.skip=false`)
- Elixir Hex.pm: skip docs during publish for NIF packages

## [1.0.0-rc.3] - 2026-03-16

### Fixed

- `build.rs` graceful fallback when `sources/language_definitions.json` missing
- Parser binaries: `TSLP_LANGUAGES` set, correct output path for `.so`/`.dylib`
- `parsers.json` manifest generated and uploaded to GitHub releases
- Elixir Hex.pm: added LICENSE, removed `native` from files list
- Maven Central: profile renamed `release` → `publish`
- Python wheel: `.pyi` type stubs and `py.typed` marker (PEP 561)

### Added

- Test apps (`tests/test_apps/`) for validating published packages across 12 ecosystems
- Shared JSON test fixtures for cross-language test parity
- Version sync for test_app dependency manifests

## [1.0.0-rc.2] - 2026-03-16

### Fixed

- `build.rs` no longer panics on crates.io installs without `sources/language_definitions.json`
- PEP 440 version sync for Python
- `sync_versions.py` gemspec single-quote regex

### Added

- Test apps structure and shared fixtures
- Task definitions for test-apps:smoke and test-apps:comprehensive

## [1.0.0-rc.1] - 2026-03-09

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
- Repository moved from `Goldziher/tree-sitter-language-pack` to `kreuzberg-dev/tree-sitter-language-pack`
- Node.js package renamed to `@kreuzberg/tree-sitter-language-pack`
- Java groupId changed from `io.github.tree-sitter` to `dev.kreuzberg`
- Go module path updated to `github.com/kreuzberg-dev/tree-sitter-language-pack/go`
- README branding updated with kreuzberg.dev banner and Discord community link

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
