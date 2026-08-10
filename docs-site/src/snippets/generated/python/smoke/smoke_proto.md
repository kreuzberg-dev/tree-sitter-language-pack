```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'syntax = "proto3";'
    config = {"language": "proto"}
    _ = process(source, config)

main()

```
