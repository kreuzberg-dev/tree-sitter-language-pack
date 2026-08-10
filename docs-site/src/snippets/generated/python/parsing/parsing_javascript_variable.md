```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "const x = 1;"
    config = {"language": "javascript"}
    _ = process(source, config)

main()

```
