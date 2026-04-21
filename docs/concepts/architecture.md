---
title: Architecture
description: "How tree-sitter-language-pack is structured: Rust core, thin binding layer, and the download system."
---

tree-sitter-language-pack follows a layered architecture: a single Rust core library handles all parsing logic, and thin binding layers expose that API natively in each target language. No business logic lives in the bindings — they are pure translation layers.

---

## High-Level Diagram

```mermaid
graph TD
    subgraph Bindings["Language Bindings"]
        PY["Python<br/>(PyO3 / maturin)"]
        NODE["Node.js<br/>(NAPI-RS)"]
        RB["Ruby<br/>(Magnus)"]
        EL["Elixir<br/>(Rustler NIF)"]
        PHP["PHP<br/>(ext-php-rs)"]
        WASM["WebAssembly<br/>(wasm-bindgen)"]
        FFI["C FFI<br/>(cbindgen)"]
    end

    subgraph FFIConsumers["FFI Consumers"]
        GO["Go<br/>(cgo)"]
        JAVA["Java<br/>(Panama FFM)"]
        CS["C# / .NET<br/>(P/Invoke)"]
    end

    subgraph Core["Rust Core (ts-pack-core)"]
        DL["Download Manager"]
        CACHE["Parser Cache"]
        PROC["Code Intelligence Engine"]
        CHUNK["Chunker"]
        TS["tree-sitter runtime"]
    end

    subgraph Parsers["Parser Binaries (remote)"]
        MANIFEST["parsers.json manifest"]
        BIN["Platform-specific .so / .dll / .dylib"]
    end

    PY --> Core
    NODE --> Core
    RB --> Core
    EL --> Core
    PHP --> Core
    WASM --> Core
    FFI --> Core
    GO --> FFI
    JAVA --> FFI
    CS --> FFI

    DL -->|"HTTPS download"| MANIFEST
    DL -->|"fetch binary"| BIN
    CACHE -->|"dlopen"| BIN
    Core --> TS
```

---

## Rust Core

All logic lives in a single crate: `crates/ts-pack-core`.

| Component | Responsibility |
|-----------|---------------|
| **Download Manager** | Resolves the remote manifest, fetches platform-specific parser binaries, stores them in the local cache. |
| **Parser Cache** | Maps language names to loaded `tree_sitter::Language` values. Once loaded, a parser is reused without re-reading from disk. |
| **Code Intelligence Engine** | Runs tree-sitter queries against a parsed tree to extract structure, imports, exports, symbols, comments, and docstrings. |
| **Chunker** | Walks the syntax tree and splits source code at natural boundaries, respecting a configurable token budget. |

The core has no language-specific code. It calls tree-sitter through its stable C ABI using dynamically loaded parser binaries.

---

## Binding Layer

Each binding is a thin crate that:

1. Calls Rust core functions.
2. Converts Rust types to the host language's native types (`String` → `str`, `Vec<T>` → list/array, `Result<T, E>` → exception/error).
3. Exposes an idiomatic API matching the host language's conventions.

Binding crates contain no parsing logic, no query definitions, and no chunking code.

| Crate | Framework | Distribution |
|-------|-----------|--------------|
| `ts-pack-python` | PyO3 + maturin | PyPI wheels |
| `ts-pack-node` | NAPI-RS | npm (multi-platform) |
| `ts-pack-ruby` | Magnus | RubyGems native gem |
| `ts-pack-elixir` | Rustler NIF | Hex.pm |
| `ts-pack-php` | ext-php-rs | Packagist |
| `ts-pack-wasm` | wasm-bindgen | npm (WASM) |
| `ts-pack-ffi` | cbindgen (C FFI) | GitHub releases |
| `packages/go` | cgo | Go modules |
| `crates/ts-pack-java` | Panama FFM | Maven Central |
| `packages/csharp` | P/Invoke | NuGet |

---

## Parser Binaries

Tree-sitter parsers are **not** compiled into the package. Instead:

1. A `parsers.json` manifest (hosted on GitHub releases) lists all 306 languages with per-platform download URLs.
2. On first use, the matching binary is downloaded and written to the local cache directory.
3. The binary is opened at runtime via `dlopen` / `LoadLibrary` and the `tree_sitter_<language>` symbol is resolved.

This keeps installation fast and download sizes minimal. See [Download Model](download-model.md) for the full detail.

---

## Repository Layout

```text
tree-sitter-language-pack/
├── crates/
│   ├── ts-pack-core/       # Rust core library
│   ├── ts-pack-python/     # Python (PyO3) binding
│   ├── ts-pack-node/       # Node.js (NAPI-RS) binding
│   ├── ts-pack-ruby/       # Ruby (Magnus) binding
│   ├── ts-pack-elixir/     # Elixir (Rustler) NIF
│   ├── ts-pack-php/        # PHP (ext-php-rs) extension
│   ├── ts-pack-wasm/       # WebAssembly (wasm-bindgen) binding
│   ├── ts-pack-ffi/        # C FFI for Go / Java / C#
│   ├── ts-pack-java/       # Java (Panama FFM) binding
│   └── ts-pack-cli/        # CLI binary
├── packages/
│   ├── go/                 # Go module (cgo wrapper)
│   ├── csharp/             # C# / .NET package (P/Invoke)
│   └── php/                # PHP Composer wrapper
├── sources/
│   └── language_definitions.json  # Grammar source registry
├── scripts/
│   └── generate_readme.py  # README sync tooling
└── tools/
    └── e2e-generator/      # Test suite generator
```

---

## Design Principles

- **Single source of truth** — All parsing and intelligence logic lives in `ts-pack-core`. Binding crates are pure glue.
- **On-demand downloads** — Parsers are not shipped in the package. They are fetched and cached per-platform when first needed.
- **ABI stability** — The C FFI layer (`ts-pack-ffi`) follows strict semantic versioning. Go, Java, and C# bindings depend on a stable C ABI, not Rust internals.
- **Zero duplication** — Query definitions, chunking strategies, and intelligence extraction are each written once in Rust and reused across all 11 language surfaces.
