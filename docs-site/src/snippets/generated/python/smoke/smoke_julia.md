---
id: fixture_python_smoke_julia
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "function main() end"
    config = {"language": "julia"}
    _ = process(source, config)

main()

```
