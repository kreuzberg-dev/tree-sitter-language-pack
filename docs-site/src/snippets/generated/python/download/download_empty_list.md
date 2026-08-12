---
id: fixture_python_download_empty_list
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import download

def main() -> None:
    names = []
    _ = download(names)

main()

```
