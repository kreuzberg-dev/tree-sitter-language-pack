```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "def main : IO Unit := pure ()"
    config = {"language": "lean"}
    _ = process(source, config)

main()

```
