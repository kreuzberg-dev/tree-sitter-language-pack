---
id: fixture_python_smoke_rtf
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "{\\rtf1 hello}"
    config = {"language": "rtf"}
    _ = process(source, config)

main()

```
