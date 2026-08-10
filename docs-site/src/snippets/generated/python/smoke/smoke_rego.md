```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "package main\ndefault allow = false"
    config = {"language": "rego"}
    _ = process(source, config)

main()

```
