```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "<config><host>localhost</host><port>8080</port></config>"
    config = {"data_extraction": True, "language": "xml"}
    _ = process(source, config)

main()

```
