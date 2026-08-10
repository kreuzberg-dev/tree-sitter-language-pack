```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "@echo off\necho hello"
    config = {"language": "batch"}
    _ = process(source, config)

main()

```
