```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "|0100 LIT 01"
    config = {"language": "uxntal"}
    _ = process(source, config)

main()

```
