---
id: fixture_python_smoke_promela
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "init {\n}\n"
    config = {"language": "promela"}
    _ = process(source, config)

main()

```
