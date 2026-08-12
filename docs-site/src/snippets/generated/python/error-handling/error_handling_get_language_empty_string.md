---
id: fixture_python_error_handling_get_language_empty_string
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
        name = ""
        _ = get_language(name)
    except Exception as error:
        print(f"Call failed as expected: {error}")
    else:
        raise AssertionError("expected call to fail")

main()

```
