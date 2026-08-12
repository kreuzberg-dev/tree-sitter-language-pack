---
id: fixture_python_smoke_llvm_mir
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "---\nname: foo\n...\n"
    config = {"language": "llvm_mir"}
    _ = process(source, config)

main()

```
