---
id: fixture_python_smoke_aiken
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "fn main() {\n  1\n}\n"
    config = {"language": "aiken"}
    _ = process(source, config)

main()

```
