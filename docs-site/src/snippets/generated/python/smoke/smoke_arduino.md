---
id: fixture_python_smoke_arduino
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "void setup() {}"
    config = {"language": "arduino"}
    _ = process(source, config)

main()

```
