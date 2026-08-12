---
id: fixture_python_data_extraction_toml_flat
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'host = "localhost"\nport = 8080\n'
    config = {"data_extraction": True, "language": "toml"}
    _ = process(source, config)

main()

```
