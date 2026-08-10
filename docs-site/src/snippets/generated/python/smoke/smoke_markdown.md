```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "# Hello\n\nWorld"
    config = {"language": "markdown"}
    _ = process(source, config)

main()

```
