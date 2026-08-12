---
id: fixture_python_smoke_postscript
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "/hello { (Hello) show } def"
    config = {"language": "postscript"}
    _ = process(source, config)

main()

```
