```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "**bold** and *italic*"
    config = {"language": "markdown_inline"}
    _ = process(source, config)

main()

```
