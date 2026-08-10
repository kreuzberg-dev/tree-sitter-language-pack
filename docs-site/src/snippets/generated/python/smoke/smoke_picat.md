```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "main => true.\n"
    config = {"language": "picat"}
    _ = process(source, config)

main()

```
