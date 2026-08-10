```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = '{"server": {"host": "x", "port": 8080}}'
    config = {"data_extraction": True, "language": "json"}
    _ = process(source, config)

main()

```
