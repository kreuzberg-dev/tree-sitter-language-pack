---
id: fixture_python_smoke_cpp
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
    config = {"language": "cpp"}
    _ = process(source, config)

main()

```
