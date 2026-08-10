```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "module Example"
    config = {"language": "qmldir"}
    _ = process(source, config)

main()

```
