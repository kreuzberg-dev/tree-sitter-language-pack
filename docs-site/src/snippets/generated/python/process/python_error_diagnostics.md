```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "def broken(\n    pass\n"
    config = {"diagnostics": True, "language": "python"}
    _ = process(source, config)

main()

```
