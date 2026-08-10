```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "module example.com/hello\n\ngo 1.21"
    config = {"language": "gomod"}
    _ = process(source, config)

main()

```
