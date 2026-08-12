---
id: fixture_python_typescript_function_process_detail
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "import { readFile } from 'fs';\n\nfunction greet(name: string): string {\n    return `Hello, ${name}!`;\n}\n"
    config = {"language": "typescript"}
    _ = process(source, config)

main()

```
