---
id: fixture_python_smoke_capnp
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "@0xabcdef1234567890;"
    config = {"language": "capnp"}
    _ = process(source, config)

main()

```
