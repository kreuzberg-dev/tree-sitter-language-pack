```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "type Set\n"
    config = {"language": "penrose"}
    _ = process(source, config)

main()

```
