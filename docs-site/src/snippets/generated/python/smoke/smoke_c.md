---
id: fixture_python_smoke_c
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "int main() { return 0; }"
    config = {"language": "c"}
    _ = process(source, config)

main()

```
