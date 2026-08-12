---
id: fixture_python_smoke_prisma
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "model User { id Int @id }"
    config = {"language": "prisma"}
    _ = process(source, config)

main()

```
