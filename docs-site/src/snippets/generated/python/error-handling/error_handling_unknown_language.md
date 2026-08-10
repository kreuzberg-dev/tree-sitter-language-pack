```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = ""
    config = {"language": "nonexistent_xyz"}
    _ = process(source, config)

main()

```
