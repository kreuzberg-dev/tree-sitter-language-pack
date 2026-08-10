```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "[section]\nkey = value"
    config = {"language": "ini"}
    _ = process(source, config)

main()

```
