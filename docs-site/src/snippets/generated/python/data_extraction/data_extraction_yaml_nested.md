---
id: fixture_python_data_extraction_yaml_nested
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "server:\n  host: localhost\n  port: 8080\n"
    config = {"data_extraction": True, "language": "yaml"}
    _ = process(source, config)

main()

```
