---
id: fixture_python_smoke_hack
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "<?hh\nfunction main(): void {}"
    config = {"language": "hack"}
    _ = process(source, config)

main()

```
