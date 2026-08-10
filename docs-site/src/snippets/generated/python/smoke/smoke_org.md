```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "* Hello\nWorld"
    config = {"language": "org"}
    _ = process(source, config)

main()

```
