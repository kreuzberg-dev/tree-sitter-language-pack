---
id: fixture_python_smoke_cypher
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "MATCH (n) RETURN n\n"
    config = {"language": "cypher"}
    _ = process(source, config)

main()

```
