---
id: fixture_python_smoke_requirements
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "flask>=2.0"
    config = {"language": "requirements"}
    _ = process(source, config)

main()

```
