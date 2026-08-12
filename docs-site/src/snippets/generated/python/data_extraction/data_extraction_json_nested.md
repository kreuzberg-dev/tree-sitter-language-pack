---
id: fixture_python_data_extraction_json_nested
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = '{"server": {"host": "x", "port": 8080}}'
    config = {"data_extraction": True, "language": "json"}
    _ = process(source, config)

main()

```
