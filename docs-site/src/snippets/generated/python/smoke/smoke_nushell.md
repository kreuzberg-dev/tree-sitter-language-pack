```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "x"
    config = {"language": "nushell"}
    _ = process(source, config)

main()

```
