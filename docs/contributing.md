---
description: "How to contribute to tree-sitter-language-pack — adding languages, fixing bugs, improving bindings."
---

# Contributing

Contributions are welcome. This guide covers the most common contribution paths.

## Prerequisites

- [Task](https://taskfile.dev/) — the project task runner
- Rust toolchain (stable, via [rustup](https://rustup.rs/))
- Python 3.10+ and [uv](https://docs.astral.sh/uv/)
- Node.js 18+ and [pnpm](https://pnpm.io/)

```bash
# Install Task
brew install go-task     # macOS
apt install go-task      # Debian/Ubuntu

# Clone the repository
git clone https://github.com/kreuzberg-dev/tree-sitter-language-pack.git
cd tree-sitter-language-pack

# Install all language dependencies
task setup

# Build the Rust core
task build

# Run all tests
task test
```text

## Common Tasks

```bash
task --list          # show all available tasks
task build           # build Rust core + bindings
task test            # run all test suites
task lint            # run all linters (clippy, ruff, biome, rubocop, …)
task format          # auto-format all code
task generate:e2e    # regenerate e2e test suites from fixtures
task test:e2e        # run e2e tests
```text

## Adding a Language

The most common contribution is adding a new tree-sitter grammar.

### 1. Find or create a grammar

The grammar must:

- **Be permissively licensed** (MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, ISC, or Unlicense). We do **not** accept grammars under GPL, AGPL, LGPL, MPL, or any other copyleft license. This ensures tree-sitter-language-pack can be freely used in any project without imposing license obligations on downstream users.
- Have a public Git repository.
- Produce valid `parser.c` output from `tree-sitter generate`.
- Compile cleanly on Linux, macOS, and Windows.

### 2. Add the grammar definition

Edit `sources/language_definitions.json` and add an entry:

```json
{
  "mylang": {
    "repo": "https://github.com/example/tree-sitter-mylang",
    "rev": "abc123def456",
    "branch": "main"
  }
}
```text

Fields:

| Field | Required | Description |
|-------|----------|-------------|
| `repo` | yes | Grammar repository URL |
| `rev` | yes | Exact commit SHA or tag to pin |
| `branch` | no | Branch name (for display / update tooling) |
| `directory` | no | Subdirectory within the repo containing the grammar |

Always pin to an exact commit (`rev`), not a branch tip. This ensures reproducible builds.

### 3. Build and test

```bash
# Compile the new parser
task build

# Run the test suite
task test

# Verify the parser works
ts-pack download mylang
ts-pack parse example.mylang --language mylang
```text

### 4. Add test fixtures

Add at least one fixture to `tools/e2e-generator/fixtures/`:

```json
[
  {
    "id": "mylang_basic_parse",
    "category": "basic",
    "description": "Parse a simple mylang file",
    "language": "mylang",
    "source_code": "// example mylang source",
    "assertions": {
      "tree_not_null": true,
      "has_error_nodes": false
    },
    "tags": ["smoke"]
  }
]
```text

Regenerate and run e2e tests:

```bash
task generate:e2e
task test:e2e
```text

### 5. Open a pull request

- Title: `feat: add <language> parser`
- Body: link to the upstream grammar, note any quirks or limitations

## Fixing a Bug

1. Check the [issue tracker](https://github.com/kreuzberg-dev/tree-sitter-language-pack/issues) — the bug may already be tracked.
2. Write a failing test that reproduces the issue.
3. Fix the bug in the appropriate crate.
4. Confirm all tests pass with `task test`.
5. Open a PR with a clear description of the root cause and fix.

## Improving Bindings

Binding improvements (better error messages, idiomatic API, new methods) are welcome. Each binding lives in `crates/ts-pack-<language>/`. See the [Architecture overview](concepts/architecture.md) for the crate layout.

Binding changes must:

- Not add logic that belongs in the Rust core.
- Have test coverage in the binding's native test suite.
- Follow the existing API surface documented in `docs/api-mapping.yaml`.

## Code Quality

The project uses pre-commit hooks managed by [prek](https://github.com/kreuzberg-dev/prek):

```bash
prek install
prek install --hook-type commit-msg
```text

Before committing, verify:

```bash
task lint     # zero warnings required
task test     # all tests must pass
task format   # code must be formatted
```text

## Commit Style

Follow [Conventional Commits](https://www.conventionalcommits.org/):

```text
feat: add kotlin parser
fix: correct memory layout in Java FFI array freeing
chore: update tree-sitter to 0.25
docs: add chunking guide
test: add e2e fixtures for ruby
```text

Keep commits small and focused. Each commit should represent one logical change.

## Pull Request Checklist

- [ ] `task test` passes
- [ ] `task lint` passes (zero warnings)
- [ ] New language has fixtures in `tools/e2e-generator/fixtures/`
- [ ] `task generate:e2e && task test:e2e` passes
- [ ] `task sync-versions` run if any manifest was bumped
- [ ] PR description explains the change and links related issues

## Getting Help

- [GitHub Discussions](https://github.com/kreuzberg-dev/tree-sitter-language-pack/discussions) — questions and design conversations
- [Discord](https://discord.gg/xt9WY3GnKR) — real-time chat with maintainers
- [Issue tracker](https://github.com/kreuzberg-dev/tree-sitter-language-pack/issues) — bug reports and feature requests
