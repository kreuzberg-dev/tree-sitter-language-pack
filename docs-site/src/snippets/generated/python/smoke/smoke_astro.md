---
id: fixture_python_smoke_astro
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "---\n---\n<p>hello</p>"
    config = {"language": "astro"}
    _ = process(source, config)

main()

```
