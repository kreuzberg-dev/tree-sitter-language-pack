```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "<p>hi</p>\n"
    config = {"language": "rshtml"}
    _ = process(source, config)

main()

```
