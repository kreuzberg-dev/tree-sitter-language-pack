```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "Module Main\nEnd Module"
    config = {"language": "vb"}
    _ = process(source, config)

main()

```
