```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "SECTIONS { .text : { *(.text) } }"
    config = {"language": "linkerscript"}
    _ = process(source, config)

main()

```
