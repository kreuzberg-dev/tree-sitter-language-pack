---
id: fixture_python_smoke_java
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "class Main {}"
    config = {"language": "java"}
    _ = process(source, config)

main()

```
