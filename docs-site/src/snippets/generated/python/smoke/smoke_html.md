---
id: fixture_python_smoke_html
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "<p>hello</p>"
    config = {"language": "html"}
    _ = process(source, config)

main()

```
