```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "%d %s"
    config = {"language": "printf"}
    _ = process(source, config)

main()

```
