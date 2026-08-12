---
id: fixture_python_smoke_gleam
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "pub fn main() { }"
    config = {"language": "gleam"}
    _ = process(source, config)

main()

```
