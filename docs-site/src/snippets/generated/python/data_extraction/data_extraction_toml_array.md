---
id: fixture_python_data_extraction_toml_array
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "ports = [8080, 8081, 8082]\n"
    config = {"data_extraction": True, "language": "toml"}
    _ = process(source, config)

main()

```
