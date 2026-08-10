```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "PRINT 1\n"
    config = {"language": "t32"}
    _ = process(source, config)

main()

```
