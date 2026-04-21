---
description: "Full Taskfile reference — all tasks in Taskfile.yml and the per-binding task includes."
---

# Task runner reference

The project uses [Task](https://taskfile.dev) as its build system. Run `task --list` to see available tasks at any time.

## Setup and build

| Task | What it does |
|------|-------------|
| `task setup` | Install all language dependencies (Python, Node, Go, etc.) |
| `task clone` | Clone grammar sources via `scripts/clone_vendors.py` |
| `task build` | Build the Rust core and all bindings (development mode) |
| `task build:dev` | Build in development mode (no optimizations) |
| `task build:release` | Build in release mode |
| `task clean` | Remove build artifacts |

## Testing

| Task | What it does |
|------|-------------|
| `task test` | Run all test suites |
| `task check` | Run type checks and static analysis without building |
| `task e2e:generate:all` | Regenerate E2E test code for all bindings |
| `task e2e:test:rust` | Run E2E tests for the Rust binding |
| `task e2e:generate:smoke-fixtures` | Regenerate smoke test fixtures |

Per-binding E2E generation tasks: `e2e:generate:rust`, `e2e:generate:python`, `e2e:generate:typescript`, `e2e:generate:go`, `e2e:generate:java`, `e2e:generate:elixir`, `e2e:generate:ruby`, `e2e:generate:wasm`, `e2e:generate:c`, `e2e:generate:php`, `e2e:generate:csharp`.

Each binding also has its own test task in its included taskfile (e.g. `.task/python.yml` exposes `task python:test`).

## Linting and formatting

| Task | What it does |
|------|-------------|
| `task lint` | Run all linters (clippy, ruff, biome, rubocop, etc.) |
| `task lint:licenses` | Verify all grammar licenses are permissive |
| `task format` | Auto-format all code |

## Grammar management

| Task | What it does |
|------|-------------|
| `task update` | Update dependencies |
| `task update:grammars` | Update all grammar revisions to latest upstream |
| `task update:grammars:missing` | Add missing grammars to the pin file |
| `task update:grammars:check` | Report outdated grammar pins without changing anything |

## Docs and readme generation

| Task | What it does |
|------|-------------|
| `task docs:generate:languages` | Regenerate `docs/supported-languages.md` from `language_definitions.json` |
| `task generate-readme` | Regenerate per-binding READMEs from templates |
| `task generate-readme:check` | Validate that checked-in READMEs match templates (used in CI) |
| `task generate-readme:dry-run` | Preview README diffs without writing |

## Included taskfiles

The root `Taskfile.yml` includes per-ecosystem taskfiles from `.task/`:

`rust.yml`, `python.yml`, `node.yml`, `go.yml`, `java.yml`, `elixir.yml`, `c.yml`, `ruby.yml`, `wasm.yml`, `php.yml`, `csharp.yml`, `version.yml`, `test-apps.yml`

Each exposes namespaced tasks (e.g. `python:test`, `node:build`) for working on a single binding in isolation. Run `task --list` to see the full set.
