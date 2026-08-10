```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "<%= value %>"
    config = {"language": "embeddedtemplate"}
    _ = process(source, config)

main()

```
