```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "(identifier) @name"
    config = {"language": "query"}
    _ = process(source, config)

main()

```
