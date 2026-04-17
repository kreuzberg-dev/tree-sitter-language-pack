---
title: tree-sitter-language-pack
description: "Comprehensive tree-sitter grammar compilation with bindings for Python, TypeScript, Rust, Go, Java, C#, Ruby, Elixir, PHP, and WebAssembly — 305 languages."
---

**Universal code parser for 305 languages** — parse, analyze, and chunk source code across every major programming language.

<div class="hero-badges" markdown>
[:material-lightning-bolt: Quick Start](getting-started/quickstart.md){ .md-button .md-button--primary }
[:material-package-variant: Installation](getting-started/installation.md){ .md-button }
[:fontawesome-brands-github: GitHub](https://github.com/kreuzberg-dev/tree-sitter-language-pack){ .md-button }
</div>

## Why tree-sitter-language-pack?

<div class="grid cards" markdown>

- :material-language-python:{ .lg .middle } **305 Languages**

    ---

    Parse Python, JavaScript, Rust, Go, Java, C, TypeScript, Ruby, and 240+ more with a single unified API. One dependency, every language.

    [:material-arrow-right: See all languages](languages.md)

- :material-download:{ .lg .middle } **Download on Demand**

    ---

    Parsers are downloaded and cached automatically on first use. No bloated installs — only fetch what you need.

    [:material-arrow-right: Learn about the download model](concepts/download-model.md)

- :material-code-braces:{ .lg .middle } **Code Intelligence**

    ---

    Extract functions, classes, imports, exports, comments, docstrings, and symbols — not just raw syntax trees.

    [:material-arrow-right: Code intelligence concepts](concepts/code-intelligence.md)

- :material-content-cut:{ .lg .middle } **Syntax-Aware Chunking**

    ---

    Split code at natural boundaries for LLMs. Never break a function in half or separate a decorator from its definition.

    [:material-arrow-right: Chunking for LLMs](guides/chunking.md)

- :material-translate:{ .lg .middle } **Available Everywhere**

    ---

    Python, Node.js, Rust, Go, Java, C#, Ruby, Elixir, PHP, WebAssembly, CLI — same API, all platforms.

    [:material-arrow-right: Installation guide](getting-started/installation.md)

- :material-speedometer:{ .lg .middle } **Native Performance**

    ---

    Rust core with zero-copy parsing. Tree-sitter powers production editors including Neovim, Helix, and Zed.

    [:material-arrow-right: Architecture overview](concepts/architecture.md)

- :material-magnify:{ .lg .middle } **Extraction Queries**

    ---

    Run custom tree-sitter queries and get structured results with text, metadata, and child fields.

    [:material-arrow-right: Extraction guide](guides/extraction.md)

</div>

## Quick Example

=== "Python"

    --8<-- "snippets/python/quickstart.md"

=== "Node.js"

    --8<-- "snippets/typescript/quickstart.md"

=== "Rust"

    --8<-- "snippets/rust/quickstart.md"

=== "Go"

    --8<-- "snippets/go/quickstart.md"

=== "Java"

    --8<-- "snippets/java/quickstart.md"

=== "C#"

    --8<-- "snippets/csharp/quickstart.md"

=== "Ruby"

    --8<-- "snippets/ruby/quickstart.md"

=== "Elixir"

    --8<-- "snippets/elixir/quickstart.md"

=== "PHP"

    --8<-- "snippets/php/quickstart.md"

=== "WASM"

    --8<-- "snippets/wasm/quickstart.md"

=== "CLI"

    --8<-- "snippets/cli/quickstart.md"

## Install

=== "Python"

    ```bash
    pip install tree-sitter-language-pack
    ```

    ```bash
    uv add tree-sitter-language-pack
    ```

=== "Node.js"

    ```bash
    npm install @kreuzberg/tree-sitter-language-pack
    ```

    ```bash
    pnpm add @kreuzberg/tree-sitter-language-pack
    ```

=== "Rust"

    ```bash
    cargo add ts-pack-core
    ```

=== "Go"

    ```bash
    go get github.com/kreuzberg-dev/tree-sitter-language-pack/packages/go
    ```

=== "Java"

    ```xml
    <dependency>
      <groupId>dev.kreuzberg</groupId>
      <artifactId>tree-sitter-language-pack</artifactId>
      <version>1.3.0</version>
    </dependency>
    ```

=== "CLI"

    ```bash
    # Homebrew (macOS / Linux)
    brew install kreuzberg-dev/tap/ts-pack
    ```

    ```bash
    # Cargo
    cargo install ts-pack
    ```

## Supported Ecosystems

| Ecosystem | Package | Minimum Version |
|-----------|---------|-----------------|
| :fontawesome-brands-python: Python | [`tree-sitter-language-pack`](https://pypi.org/project/tree-sitter-language-pack/) | 3.10+ |
| :fontawesome-brands-js: Node.js | [`@kreuzberg/tree-sitter-language-pack`](https://www.npmjs.com/package/@kreuzberg/tree-sitter-language-pack) | 18+ |
| :fontawesome-brands-rust: Rust | [`ts-pack-core`](https://crates.io/crates/tree-sitter-language-pack) | 1.75+ |
| :fontawesome-brands-golang: Go | [`packages/go`](https://github.com/kreuzberg-dev/tree-sitter-language-pack/tree/main/packages/go) | 1.26+ |
| :fontawesome-brands-java: Java | [`dev.kreuzberg:tree-sitter-language-pack`](https://central.sonatype.com/artifact/dev.kreuzberg/tree-sitter-language-pack) | 25+ |
| :material-language-csharp: C# / .NET | [`TreeSitterLanguagePack`](https://www.nuget.org/packages/TreeSitterLanguagePack) | .NET 10+ |
| :material-language-ruby: Ruby | [`tree_sitter_language_pack`](https://rubygems.org/gems/tree_sitter_language_pack) | 3.4+ |
| :simple-elixir: Elixir | [`tree_sitter_language_pack`](https://hex.pm/packages/tree_sitter_language_pack) | 1.14+ / OTP 25+ |
| :material-elephant: PHP | [`kreuzberg/tree-sitter-language-pack`](https://packagist.org/packages/kreuzberg/tree-sitter-language-pack) | 8.2+ |
| :material-web: WebAssembly | [`@kreuzberg/tree-sitter-language-pack-wasm`](https://www.npmjs.com/package/@kreuzberg/tree-sitter-language-pack-wasm) | Browser / Deno |
| :material-console: CLI | [`ts-pack`](https://github.com/kreuzberg-dev/homebrew-tap) | — |

---

tree-sitter-language-pack is part of the [kreuzberg.dev](https://kreuzberg.dev) open-source ecosystem.
