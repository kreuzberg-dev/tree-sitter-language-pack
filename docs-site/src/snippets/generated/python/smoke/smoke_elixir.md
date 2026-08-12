---
id: fixture_python_smoke_elixir
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'IO.puts("hello")'
    config = {"language": "elixir"}
    _ = process(source, config)

main()

```
