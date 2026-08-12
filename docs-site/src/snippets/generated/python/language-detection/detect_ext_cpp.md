---
id: fixture_python_detect_ext_cpp
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import detect_language_from_extension

def main() -> None:
    ext = "cpp"
    _ = detect_language_from_extension(ext)

main()

```
