---
id: fixture_python_data_extraction_po_message
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'msgid "Hello"\nmsgstr "Hallo"\n'
    config = {"data_extraction": True, "language": "po"}
    _ = process(source, config)

main()

```
