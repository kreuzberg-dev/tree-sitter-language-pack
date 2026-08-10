```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'host "localhost"\nport 8080\n'
    config = {"data_extraction": True, "language": "kdl"}
    _ = process(source, config)

main()

```
