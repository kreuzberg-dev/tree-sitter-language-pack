```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "version 1.0\n"
    config = {"language": "wdl"}
    _ = process(source, config)

main()

```
