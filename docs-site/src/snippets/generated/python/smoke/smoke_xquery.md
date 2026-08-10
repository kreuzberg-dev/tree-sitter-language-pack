```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "1\n"
    config = {"language": "xquery"}
    _ = process(source, config)

main()

```
