```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "%p hello\n"
    config = {"language": "haml"}
    _ = process(source, config)

main()

```
