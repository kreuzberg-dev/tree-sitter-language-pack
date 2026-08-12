---
id: fixture_python_error_detect_content_empty
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import detect_language_from_content

def main() -> None:
    content = ""
    _ = detect_language_from_content(content)

main()

```
