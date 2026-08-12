---
id: fixture_python_smoke_ballerina
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "public function main() {\n}\n"
    config = {"language": "ballerina"}
    _ = process(source, config)

main()

```
