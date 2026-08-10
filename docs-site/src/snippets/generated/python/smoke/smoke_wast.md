```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "(module)"
    config = {"language": "wast"}
    _ = process(source, config)

main()

```
