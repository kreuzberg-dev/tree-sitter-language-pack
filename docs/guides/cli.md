---
description: "CLI reference for ts-pack — download parsers, parse files, extract code intelligence, and manage the cache."
---

# CLI Reference

`ts-pack` is the command-line interface for tree-sitter-language-pack. Use it to download parsers, parse files, extract code intelligence, and manage the local cache.

## Installation

=== "Homebrew (macOS / Linux)"

    ```bash
    brew install kreuzberg-dev/tap/ts-pack
    ```

=== "Cargo"

    ```bash
    cargo install ts-pack
    ```

=== "Install script"

    ```bash
    curl -fsSL https://raw.githubusercontent.com/kreuzberg-dev/tree-sitter-language-pack/main/install.sh | bash
    ```

Verify:

```bash
ts-pack --version
# ts-pack 1.0.0
```text

## Global Flags

These flags apply to all commands:

| Flag | Description |
|------|-------------|
| `--cache-dir <path>` | Override the cache directory |
| `--config <path>` | Path to `language-pack.toml` (default: search from cwd) |
| `--verbose` / `-v` | Enable verbose output |
| `--quiet` / `-q` | Suppress all output except errors |
| `--no-color` | Disable ANSI color output |
| `--help` / `-h` | Show help for any command |
| `--version` / `-V` | Print version |

## Commands

### `ts-pack list`

List available languages.

```bash
ts-pack list [OPTIONS]
```text

**Options:**

| Flag | Description |
|------|-------------|
| `--manifest` | List all languages in the remote manifest (default) |
| `--downloaded` | List only locally cached parsers |
| `--format <fmt>` | Output format: `text` (default) or `json` |

**Examples:**

```bash
# List all 305 languages
ts-pack list

# List only what's downloaded locally
ts-pack list --downloaded

# Count how many are available
ts-pack list | wc -l

# Get the list as JSON
ts-pack list --format json | jq '.[]'
```text

---

### `ts-pack download`

Download parser binaries to the local cache.

```bash
ts-pack download [LANGUAGES...] [OPTIONS]
```text

**Options:**

| Flag | Description |
|------|-------------|
| `--all` | Download all 305 parsers |
| `--force` | Re-download even if already cached |

**Examples:**

```bash
# Download specific parsers
ts-pack download python javascript typescript

# Download everything (~150 MB)
ts-pack download --all

# Force re-download (useful after a version upgrade)
ts-pack download python --force

# Download all languages from language-pack.toml
ts-pack download
```text

---

### `ts-pack clean`

Remove cached parser binaries.

```bash
ts-pack clean [LANGUAGES...] [OPTIONS]
```text

**Options:**

| Flag | Description |
|------|-------------|
| `--all` | Remove all cached parsers (default if no languages specified) |

**Examples:**

```bash
# Remove all cached parsers
ts-pack clean

# Remove only specific parsers
ts-pack clean python javascript

# Confirm before removing
ts-pack clean --all  # prompts unless --quiet
```text

---

### `ts-pack cache-dir`

Print the effective cache directory path.

```bash
ts-pack cache-dir
```text

**Example:**

```bash
ts-pack cache-dir
# /home/user/.cache/tree-sitter-language-pack

# Use in scripts
CACHE=$(ts-pack cache-dir)
du -sh "$CACHE"
```text

---

### `ts-pack parse`

Parse source code and output the syntax tree.

```bash
ts-pack parse <FILE> [OPTIONS]
ts-pack parse --language <LANG> [OPTIONS]   # reads from stdin
```text

**Options:**

| Flag | Description |
|------|-------------|
| `--language <lang>` | Language name (auto-detected from extension if not set) |
| `--format <fmt>` | Output format: `sexp` (default), `json`, or `pretty` |
| `--show-errors` | Highlight error nodes in output |

**Examples:**

```bash
# Parse a file (language auto-detected)
ts-pack parse src/main.py

# Output as JSON
ts-pack parse src/main.py --format json

# Parse from stdin
echo "def hello(): pass" | ts-pack parse --language python

# Parse with error highlighting
ts-pack parse broken.js --show-errors

# Pretty-print the tree
ts-pack parse src/main.rs --format pretty
```text

**Sample output (`--format sexp`):**

```text
(module
  (function_definition
    name: (identifier)
    parameters: (parameters)
    body: (block
      (expression_statement
        (call ...)))))
```text

---

### `ts-pack process`

Run code intelligence on a source file and output structured results.

```bash
ts-pack process <FILE> [OPTIONS]
ts-pack process --language <LANG> [OPTIONS]   # reads from stdin
```text

**Options:**

| Flag | Description |
|------|-------------|
| `--language <lang>` | Language name (auto-detected if not set) |
| `--all` | Enable all extraction features |
| `--structure` | Extract functions, classes, methods |
| `--imports` | Extract import statements |
| `--exports` | Extract exported symbols |
| `--comments` | Extract comments |
| `--docstrings` | Extract docstrings |
| `--symbols` | Extract all identifiers |
| `--diagnostics` | Report syntax errors |
| `--chunk-size <n>` | Split output into chunks of `n` tokens |
| `--chunk-overlap <n>` | Overlap `n` tokens between adjacent chunks |
| `--format <fmt>` | Output format: `json` (default) or `text` |

**Examples:**

```bash
# Full intelligence on a Python file
ts-pack process src/app.py --all

# Extract only structure
ts-pack process src/app.py --structure --format json

# Chunk a large file for LLM ingestion
ts-pack process large_module.py --chunk-size 800 --format json \
  | jq '.chunks[] | {start: .start_line, end: .end_line, tokens: .token_count}'

# Get function names
ts-pack process src/lib.rs --structure --format json \
  | jq '.structure[] | select(.kind == "function") | .name'

# Pipe from another command
cat src/main.go | ts-pack process --language go --imports
```text

---

### `ts-pack init`

Initialize a `language-pack.toml` configuration file in the current directory.

```bash
ts-pack init [OPTIONS]
```text

**Options:**

| Flag | Description |
|------|-------------|
| `--languages <langs>` | Comma-separated list of languages to add |

**Example:**

```bash
ts-pack init
# Creates language-pack.toml

ts-pack init --languages python,javascript,typescript,rust
```text

**Generated `language-pack.toml`:**

```toml
[pack]
# Languages to pre-download (run: ts-pack download)
languages = ["python", "javascript", "typescript", "rust"]

# Optional: use a project-local cache directory
# cache_dir = ".cache/parsers"
```text

---

### `ts-pack status`

Show the download status of all configured languages (from `language-pack.toml`).

```bash
ts-pack status
```text

**Example output:**

```text
Language         Status
─────────────────────────────────
python           ✓ cached
javascript       ✓ cached
typescript       ✓ cached
rust             ✗ not downloaded
go               ✗ not downloaded
```text

---

### `ts-pack completions`

Generate shell completion scripts.

```bash
ts-pack completions <SHELL>
```text

**Supported shells:** `bash`, `zsh`, `fish`, `powershell`, `elvish`

**Examples:**

```bash
# Bash
ts-pack completions bash >> ~/.bash_completion

# Zsh
ts-pack completions zsh > ~/.zsh/completions/_ts-pack

# Fish
ts-pack completions fish > ~/.config/fish/completions/ts-pack.fish
```text

## Environment Variables

| Variable | Description |
|----------|-------------|
| `TSLP_CACHE_DIR` | Override the default cache directory |
| `TSLP_CONFIG` | Path to `language-pack.toml` |
| `TSLP_NO_COLOR` | Disable color output (same as `--no-color`) |
| `TSLP_VERBOSE` | Enable verbose output (same as `--verbose`) |

## Exit Codes

| Code | Meaning |
|------|---------|
| `0` | Success |
| `1` | General error |
| `2` | Language not found |
| `3` | Download failed |
| `4` | Parse error |
| `5` | File not found |

## Usage in CI

```yaml
# GitHub Actions — pre-download parsers and run intelligence
- name: Install ts-pack
  run: brew install kreuzberg-dev/tap/ts-pack

- name: Cache parsers
  uses: actions/cache@v4
  with:
    path: ~/.cache/tree-sitter-language-pack
    key: tslp-${{ hashFiles('language-pack.toml') }}

- name: Download parsers
  run: ts-pack download

- name: Analyze code
  run: |
    ts-pack process src/ --structure --format json > analysis.json
    jq '.structure | length' analysis.json
```
