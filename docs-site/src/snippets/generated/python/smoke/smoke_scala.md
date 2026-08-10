```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "object Main"
    config = {"language": "scala"}
    _ = process(source, config)

main()

```
