---
id: fixture_python_smoke_smali
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = ".class public LMain;\n.super Ljava/lang/Object;"
    config = {"language": "smali"}
    _ = process(source, config)

main()

```
