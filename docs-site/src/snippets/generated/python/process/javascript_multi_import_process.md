---
id: fixture_python_javascript_multi_import_process
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "import fs from 'fs';\nimport path from 'path';\n\nfunction process(input) {\n    return input.trim();\n}\n"
    config = {"language": "javascript"}
    _ = process(source, config)

main()

```
