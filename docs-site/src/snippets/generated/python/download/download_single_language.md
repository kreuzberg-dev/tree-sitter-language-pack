---
id: fixture_python_download_single_language
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import download

def main() -> None:
    names = ["python"]
    _ = download(names)

main()

```
