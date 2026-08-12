---
id: fixture_python_download_configure_custom_dir
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import configure

def main() -> None:
    config = {"cache_dir": "/tmp/tslp_test_cache"}  # noqa: S108
    _ = configure(config)

main()

```
