---
id: fixture_python_smoke_twig
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "{{ variable }}"
    config = {"language": "twig"}
    _ = process(source, config)

main()

```
