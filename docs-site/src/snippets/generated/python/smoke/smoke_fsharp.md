```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "let x = 1"
    config = {"language": "fsharp"}
    _ = process(source, config)

main()

```
