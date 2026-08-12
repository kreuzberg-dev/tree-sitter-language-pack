---
id: fixture_python_parsing_javascript_class
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "class Foo { bar() {} }"
    config = {"language": "javascript"}
    _ = process(source, config)

main()

```
