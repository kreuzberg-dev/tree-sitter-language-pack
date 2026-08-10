```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "===========\nTest\n===========\n---\n(node)"
    config = {"language": "test"}
    _ = process(source, config)

main()

```
