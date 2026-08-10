```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "foo = 1\n"
    config = {"language": "fusion"}
    _ = process(source, config)

main()

```
