```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "1. e4 e5 *"
    config = {"language": "pgn"}
    _ = process(source, config)

main()

```
