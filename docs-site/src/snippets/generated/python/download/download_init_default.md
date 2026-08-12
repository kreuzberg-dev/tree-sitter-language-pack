---
id: fixture_python_download_init_default
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import init

def main() -> None:
    config = {}
    _ = init(config)

main()

```
