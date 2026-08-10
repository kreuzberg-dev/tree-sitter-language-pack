```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "module m {\n}\n"
    config = {"language": "yang"}
    _ = process(source, config)

main()

```
