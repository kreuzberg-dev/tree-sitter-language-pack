---
id: fixture_python_python_multi_import_process
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "import os\nimport sys\nfrom pathlib import Path\n\ndef main():\n    pass\n"
    config = {"language": "python"}
    _ = process(source, config)

main()

```
