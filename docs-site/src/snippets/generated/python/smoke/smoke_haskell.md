---
id: fixture_python_smoke_haskell
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'main = putStrLn "hello"'
    config = {"language": "haskell"}
    _ = process(source, config)

main()

```
