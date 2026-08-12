---
id: fixture_python_smoke_elisp
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = '(defun hello () (message "hello"))'
    config = {"language": "elisp"}
    _ = process(source, config)

main()

```
