```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = '<server id="main"><host>localhost</host></server>'
    config = {"data_extraction": True, "language": "xml"}
    _ = process(source, config)

main()

```
