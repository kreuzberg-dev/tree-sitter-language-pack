```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "@doc foo\n"
    config = {"language": "edoc"}
    _ = process(source, config)

main()

```
