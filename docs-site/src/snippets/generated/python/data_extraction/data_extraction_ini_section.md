```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "[database]\nhost=localhost\nport=5432\n"
    config = {"data_extraction": True, "language": "ini"}
    _ = process(source, config)

main()

```
