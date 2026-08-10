```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "[a-z]+"
    config = {"language": "luap"}
    _ = process(source, config)

main()

```
