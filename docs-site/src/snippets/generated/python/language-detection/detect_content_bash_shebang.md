---
id: fixture_python_detect_content_bash_shebang
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import detect_language_from_content

def main() -> None:
    content = "#!/bin/bash\necho hi"
    _ = detect_language_from_content(content)

main()

```
