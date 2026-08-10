```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'print("hello")'
    config = {"language": "swift"}
    _ = process(source, config)

main()

```
