---
id: fixture_python_smoke_smithy
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "namespace example\nstring MyString"
    config = {"language": "smithy"}
    _ = process(source, config)

main()

```
