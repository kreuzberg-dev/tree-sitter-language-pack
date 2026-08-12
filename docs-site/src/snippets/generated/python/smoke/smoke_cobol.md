---
id: fixture_python_smoke_cobol
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "       IDENTIFICATION DIVISION.\n       PROGRAM-ID. HELLO."
    config = {"language": "cobol"}
    _ = process(source, config)

main()

```
