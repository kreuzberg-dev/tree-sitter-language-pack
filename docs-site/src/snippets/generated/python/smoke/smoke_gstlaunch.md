---
id: fixture_python_smoke_gstlaunch
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "fakesrc ! fakesink"
    config = {"language": "gstlaunch"}
    _ = process(source, config)

main()

```
