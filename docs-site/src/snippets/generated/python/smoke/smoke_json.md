```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = '{"key": "value"}'
    config = {"language": "json"}
    _ = process(source, config)

main()

```
