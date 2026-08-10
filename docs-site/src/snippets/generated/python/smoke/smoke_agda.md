```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "module Main where"
    config = {"language": "agda"}
    _ = process(source, config)

main()

```
