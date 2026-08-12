---
id: fixture_python_smoke_flatbuffers
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "table Foo {}\n"
    config = {"language": "flatbuffers"}
    _ = process(source, config)

main()

```
