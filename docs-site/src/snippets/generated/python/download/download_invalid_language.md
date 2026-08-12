---
id: fixture_python_download_invalid_language
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import download

def main() -> None:
    try:
        names = ["zzz_definitely_not_a_real_language_xyz"]
        _ = download(names)
    except Exception as error:
        print(f"Call failed as expected: {error}")
    else:
        raise AssertionError("expected call to fail")

main()

```
