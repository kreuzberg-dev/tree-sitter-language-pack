---
id: fixture_python_data_extraction_json_array
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "[1, 2, 3]"
    config = {"data_extraction": True, "language": "json"}
    _ = process(source, config)

main()

```
