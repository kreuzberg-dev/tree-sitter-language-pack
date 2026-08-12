---
id: fixture_python_smoke_postgres
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "SELECT 1;\n"
    config = {"language": "postgres"}
    _ = process(source, config)

main()

```
