---
id: fixture_python_smoke_cuda
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "__global__ void kernel() {}"
    config = {"language": "cuda"}
    _ = process(source, config)

main()

```
