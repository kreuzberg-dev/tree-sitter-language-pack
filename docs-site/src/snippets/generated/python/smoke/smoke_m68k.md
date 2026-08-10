```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = " move.l d0,d1\n"
    config = {"language": "m68k"}
    _ = process(source, config)

main()

```
