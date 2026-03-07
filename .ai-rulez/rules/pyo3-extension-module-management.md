---
priority: critical
---

# PyO3 Extension Module Management

The `extension-module` feature in `crates/spikard-py/Cargo.toml` must NOT be in default
features—it breaks linking for binaries that embed Python (like spikard-cli). Configure
maturin in `pyproject.toml` with `features = ["extension-module"]` so Python extension
modules build correctly. Binaries (CLI, tests) build without extension-module to link
libpython; extensions (maturin builds) enable it for manylinux compliance.
