---
id: fixture_python_python_malformed_code_process_detail
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "def broken(\n    return\nclass"
    config = {"diagnostics": True, "language": "python"}
    _ = process(source, config)

main()

```
