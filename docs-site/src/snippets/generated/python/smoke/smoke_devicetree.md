---
id: fixture_python_smoke_devicetree
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "/dts-v1/;\n/ { };"
    config = {"language": "devicetree"}
    _ = process(source, config)

main()

```
