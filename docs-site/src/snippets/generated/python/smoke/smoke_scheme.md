```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "(define x 1)"
    config = {"language": "scheme"}
    _ = process(source, config)

main()

```
