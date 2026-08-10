```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "graph TD\nA --> B"
    config = {"language": "mermaid"}
    _ = process(source, config)

main()

```
