```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "set x to 1\n"
    config = {"language": "applescript"}
    _ = process(source, config)

main()

```
