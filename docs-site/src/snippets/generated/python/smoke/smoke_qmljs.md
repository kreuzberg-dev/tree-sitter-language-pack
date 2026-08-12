---
id: fixture_python_smoke_qmljs
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "import QtQuick 2.0\nItem {}"
    config = {"language": "qmljs"}
    _ = process(source, config)

main()

```
