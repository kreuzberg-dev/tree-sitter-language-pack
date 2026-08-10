```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "server:\n  host: localhost\n  port: 8080\n"
    config = {"data_extraction": True, "language": "yaml"}
    _ = process(source, config)

main()

```
