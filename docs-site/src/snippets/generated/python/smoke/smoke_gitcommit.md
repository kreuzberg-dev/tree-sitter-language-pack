---
id: fixture_python_smoke_gitcommit
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "feat: add feature\n\nBody text"
    config = {"language": "gitcommit"}
    _ = process(source, config)

main()

```
