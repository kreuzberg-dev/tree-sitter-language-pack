---
id: fixture_python_download_multiple_languages
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import download

def main() -> None:
    names = ["python", "rust"]
    _ = download(names)

main()

```
