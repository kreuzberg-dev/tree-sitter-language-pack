---
id: fixture_python_data_extraction_csv_rows
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "a,b,c\n1,2,3\n"
    config = {"data_extraction": True, "language": "csv"}
    _ = process(source, config)

main()

```
