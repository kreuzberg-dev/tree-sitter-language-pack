```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "entity User;"
    config = {"language": "cedarschema"}
    _ = process(source, config)

main()

```
