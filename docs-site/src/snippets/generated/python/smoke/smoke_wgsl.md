---
id: fixture_python_smoke_wgsl
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "@vertex fn main() -> @builtin(position) vec4f { return vec4f(); }"
    config = {"language": "wgsl"}
    _ = process(source, config)

main()

```
