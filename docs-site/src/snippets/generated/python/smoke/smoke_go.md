```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "package main"
    config = {"language": "go"}
    _ = process(source, config)

main()

```
