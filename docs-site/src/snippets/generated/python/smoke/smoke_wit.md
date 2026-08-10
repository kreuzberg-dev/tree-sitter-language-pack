```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "package example:pkg;"
    config = {"language": "wit"}
    _ = process(source, config)

main()

```
