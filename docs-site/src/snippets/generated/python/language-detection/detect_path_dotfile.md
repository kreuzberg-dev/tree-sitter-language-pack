---
id: fixture_python_detect_path_dotfile
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import detect_language_from_path

def main() -> None:
    path = ".gitignore"
    _ = detect_language_from_path(path)

main()

```
