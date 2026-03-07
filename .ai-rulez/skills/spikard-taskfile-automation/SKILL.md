---
priority: high
description: "Task Automation & Build Orchestration"
---

# Task Automation & Build Orchestration

**Taskfile.yaml · Multi-language coordination · Dependency management · CI/CD parity**

Root Commands: `task setup` (install tooling, build bindings), `task update` (upgrade all), `task build` (all languages), `task lint` (mypy, clippy, biome, steep, phpstan), `task format` (all), `task test` (all suites).

Language-Specific: `task rust:build`, `task python:build` (maturin), `task python:test`, `task js:build`, `task js:test`, `task ruby:build`, `task php:build`, `task wasm:build`.

Dependency Files (committed): Cargo.lock, uv.lock, pnpm-lock.yaml, Gemfile.lock, composer.lock. All mandatory in version control.
