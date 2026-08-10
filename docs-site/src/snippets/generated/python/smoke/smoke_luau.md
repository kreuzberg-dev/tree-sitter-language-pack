```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "local x: number = 1"
    config = {"language": "luau"}
    _ = process(source, config)

main()

```
