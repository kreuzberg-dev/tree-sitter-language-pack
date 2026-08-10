```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "puts 'hello'"
    config = {"language": "ruby"}
    _ = process(source, config)

main()

```
