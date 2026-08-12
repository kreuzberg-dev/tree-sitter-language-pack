---
id: fixture_python_data_extraction_editorconfig_section
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "[*.rs]\nindent_style = space\nindent_size = 4\n"
    config = {"data_extraction": True, "language": "editorconfig"}
    _ = process(source, config)

main()

```
