---
id: fixture_python_smoke_markdown
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "# Hello\n\nWorld"
    config = {"language": "markdown"}
    _ = process(source, config)

main()

```
