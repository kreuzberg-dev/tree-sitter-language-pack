---
id: fixture_python_smoke_avro
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "protocol P {\n}\n"
    config = {"language": "avro"}
    _ = process(source, config)

main()

```
