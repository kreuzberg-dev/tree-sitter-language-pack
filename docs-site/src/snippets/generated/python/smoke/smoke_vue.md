---
id: fixture_python_smoke_vue
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "<template><div>hello</div></template>"
    config = {"language": "vue"}
    _ = process(source, config)

main()

```
