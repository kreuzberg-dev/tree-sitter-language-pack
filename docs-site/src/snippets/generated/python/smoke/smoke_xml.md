```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = '<?xml version="1.0"?>\n<root>hello</root>'
    config = {"language": "xml"}
    _ = process(source, config)

main()

```
