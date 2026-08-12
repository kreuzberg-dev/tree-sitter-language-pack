---
id: fixture_python_error_handling_unknown_language
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    try:
        source = ""
        config = {"language": "nonexistent_xyz"}
        _ = process(source, config)
    except Exception as error:
        print(f"Call failed as expected: {error}")
    else:
        raise AssertionError("expected call to fail")

main()

```
