---
id: fixture_python_get_parser_unknown
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import get_parser

def main() -> None:
    try:
        name = "nonexistent_xyz"
        _ = get_parser(name)
    except Exception as error:
        print(f"Call failed as expected: {error}")
    else:
        raise AssertionError("expected call to fail")

main()

```
