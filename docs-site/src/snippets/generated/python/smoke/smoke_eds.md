```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "x"
    config = {"language": "eds"}
    _ = process(source, config)

main()

```
