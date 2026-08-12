---
id: fixture_python_process_python_comments
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "# This is a comment\n# Another comment\ndef hello():\n    # inline comment\n    pass\n"
    config = {"comments": True, "language": "python"}
    _ = process(source, config)

main()

```
