---
id: fixture_python_smoke_dotenv
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "KEY=value\n"
    config = {"language": "dotenv"}
    _ = process(source, config)

main()

```
