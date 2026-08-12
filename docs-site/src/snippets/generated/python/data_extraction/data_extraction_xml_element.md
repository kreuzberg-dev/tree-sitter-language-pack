---
id: fixture_python_data_extraction_xml_element
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = '<server id="main"><host>localhost</host></server>'
    config = {"data_extraction": True, "language": "xml"}
    _ = process(source, config)

main()

```
