---
id: fixture_python_process_python_symbols
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "MY_CONST = 42\ndef helper(): pass\nclass Widget: pass\n"
    config = {"language": "python", "symbols": True}
    _ = process(source, config)

main()

```
