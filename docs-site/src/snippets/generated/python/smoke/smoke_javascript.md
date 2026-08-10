```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "console.log('hello');"
    config = {"language": "javascript"}
    _ = process(source, config)

main()

```
