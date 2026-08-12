---
id: fixture_python_data_extraction_hjson_flat
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = '{\n  host: "localhost"\n  port: 8080\n}\n'
    config = {"data_extraction": True, "language": "hjson"}
    _ = process(source, config)

main()

```
