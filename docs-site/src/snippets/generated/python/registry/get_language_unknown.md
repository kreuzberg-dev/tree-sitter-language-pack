---
id: fixture_python_get_language_unknown
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import get_language

def main() -> None:
    try:
        name = "nonexistent_xyz"
        _ = get_language(name)
    except Exception as error:
        print(f"Call failed as expected: {error}")
    else:
        raise AssertionError("expected call to fail")

main()

```
