```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "server.host=localhost\nserver.port=8080\n"
    config = {"data_extraction": True, "language": "properties"}
    _ = process(source, config)

main()

```
