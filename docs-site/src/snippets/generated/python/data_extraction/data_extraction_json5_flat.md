```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = '{\n  host: "localhost",\n  port: 8080,\n}\n'
    config = {"data_extraction": True, "language": "json5"}
    _ = process(source, config)

main()

```
