```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = ""
    config = {"data_extraction": True, "language": "properties"}
    _ = process(source, config)

main()

```
