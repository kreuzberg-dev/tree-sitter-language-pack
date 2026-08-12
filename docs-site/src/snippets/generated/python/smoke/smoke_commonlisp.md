---
id: fixture_python_smoke_commonlisp
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = '(defun hello () (print "hello"))'
    config = {"language": "commonlisp"}
    _ = process(source, config)

main()

```
