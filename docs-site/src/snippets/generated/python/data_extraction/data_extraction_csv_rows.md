```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "a,b,c\n1,2,3\n"
    config = {"data_extraction": True, "language": "csv"}
    _ = process(source, config)

main()

```
