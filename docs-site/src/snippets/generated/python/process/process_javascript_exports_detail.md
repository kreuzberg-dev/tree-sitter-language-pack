---
id: fixture_python_process_javascript_exports_detail
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "export function greet(name) {\n  return `Hello ${name}`;\n}\n\nexport const VERSION = '1.0';\n"
    config = {"language": "javascript"}
    _ = process(source, config)

main()

```
