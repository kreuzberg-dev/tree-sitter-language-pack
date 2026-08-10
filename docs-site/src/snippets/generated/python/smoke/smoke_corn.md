```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = '{ key = "value" }'
    config = {"language": "corn"}
    _ = process(source, config)

main()

```
