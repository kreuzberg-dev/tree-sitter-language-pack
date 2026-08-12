---
id: fixture_python_data_extraction_ini_flat
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "host=localhost\nport=8080\n"
    config = {"data_extraction": True, "language": "ini"}
    _ = process(source, config)

main()

```
