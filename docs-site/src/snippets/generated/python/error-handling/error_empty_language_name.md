```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "hello"
    config = {"language": ""}
    _ = process(source, config)

main()

```
