```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "Root = Item*\nItem = 'token'"
    config = {"language": "ungrammar"}
    _ = process(source, config)

main()

```
