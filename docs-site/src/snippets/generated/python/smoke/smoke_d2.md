```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "a -> b\n"
    config = {"language": "d2"}
    _ = process(source, config)

main()

```
