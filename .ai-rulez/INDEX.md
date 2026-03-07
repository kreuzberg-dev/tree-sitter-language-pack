# tree-sitter-language-pack AI-Rulez Configuration Index

**Location**: `.ai-rulez/`
**Schema Version**: ai-rules-v3
**Last Updated**: 2026-03-07

## Configuration Files

| File | Description |
|------|-------------|
| [config.yaml](config.yaml) | Base configuration (name, description, includes, presets) |
| [custom-rules.yaml](custom-rules.yaml) | Project-specific rules for grammar management, build system, testing, linting |
| [custom-agents.yaml](custom-agents.yaml) | Project-specific agents for Rust core, polyglot bindings, testing, build ops |
| [custom-profiles.yaml](custom-profiles.yaml) | tree-sitter-language-pack profile composing agents and rules |
| [mcp.yaml](mcp.yaml) | MCP server configuration for ai-rulez tooling |

## Project Overview

Rust workspace providing tree-sitter language parsers with polyglot bindings:

- **Python** (PyO3) - crates/ts-pack-python
- **Node.js** (NAPI-RS) - crates/ts-pack-node
- **Elixir** (Rustler) - crates/ts-pack-elixir
- **C** (FFI) - crates/ts-pack-ffi
- **Go** (cgo) - crates/ts-pack-go
- **Java** (Panama FFI) - crates/ts-pack-java

## Key Commands

```bash
task setup          # Install deps, clone vendors, generate ai-rulez
task build          # Build all crates
task test           # Run all tests
task lint           # Lint via prek
task e2e:generate:all  # Generate E2E tests from fixtures
task generate-readme   # Generate READMEs from Jinja2 templates
```

## Integration with Shared AI-Rulez

This configuration extends shared rules from `git@github.com:kreuzberg-dev/ai-rulez.git`
via the `includes` directive in `config.yaml`, using `local-override` merge strategy.
