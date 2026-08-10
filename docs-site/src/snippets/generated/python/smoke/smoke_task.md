```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "todo item\n"
    config = {"language": "task"}
    _ = process(source, config)

main()

```
