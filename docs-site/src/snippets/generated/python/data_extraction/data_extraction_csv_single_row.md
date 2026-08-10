```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "x,y,z\n"
    config = {"data_extraction": True, "language": "csv"}
    _ = process(source, config)

main()

```
