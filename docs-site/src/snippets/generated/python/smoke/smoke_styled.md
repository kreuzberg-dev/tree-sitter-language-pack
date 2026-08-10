```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "color: red;\n"
    config = {"language": "styled"}
    _ = process(source, config)

main()

```
