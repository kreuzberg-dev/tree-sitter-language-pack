```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "<!ELEMENT note (body)>"
    config = {"language": "dtd"}
    _ = process(source, config)

main()

```
