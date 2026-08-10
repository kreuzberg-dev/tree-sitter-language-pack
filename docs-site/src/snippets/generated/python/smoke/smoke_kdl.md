```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'node "value"'
    config = {"language": "kdl"}
    _ = process(source, config)

main()

```
