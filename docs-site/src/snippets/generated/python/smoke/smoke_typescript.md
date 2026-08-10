```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "const x: number = 42;"
    config = {"language": "typescript"}
    _ = process(source, config)

main()

```
