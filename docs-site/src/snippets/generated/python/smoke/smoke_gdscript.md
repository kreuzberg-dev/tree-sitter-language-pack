```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "extends Node\nfunc _ready():\n\tpass"
    config = {"language": "gdscript"}
    _ = process(source, config)

main()

```
