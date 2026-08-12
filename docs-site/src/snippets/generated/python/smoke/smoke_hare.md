---
id: fixture_python_smoke_hare
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "export fn main() void = void;"
    config = {"language": "hare"}
    _ = process(source, config)

main()

```
