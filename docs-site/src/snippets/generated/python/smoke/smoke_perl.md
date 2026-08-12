---
id: fixture_python_smoke_perl
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "print 'hello';"
    config = {"language": "perl"}
    _ = process(source, config)

main()

```
