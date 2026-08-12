---
id: fixture_python_smoke_sflog
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "37.0 APEX_CODE,DEBUG\n16:06:58.18 (1)|EXECUTION_STARTED\n"
    config = {"language": "sflog"}
    _ = process(source, config)

main()

```
