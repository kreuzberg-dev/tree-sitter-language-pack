---
id: fixture_python_smoke_powershell
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "Write-Host 'hello'"
    config = {"language": "powershell"}
    _ = process(source, config)

main()

```
