```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "procedure Main is begin null; end Main;"
    config = {"language": "ada"}
    _ = process(source, config)

main()

```
