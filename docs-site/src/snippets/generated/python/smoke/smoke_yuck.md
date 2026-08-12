---
id: fixture_python_smoke_yuck
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = '(defwidget main [] (label :text "hi"))'
    config = {"language": "yuck"}
    _ = process(source, config)

main()

```
