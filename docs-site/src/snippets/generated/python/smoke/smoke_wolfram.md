```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "x = 1\n"
    config = {"language": "wolfram"}
    _ = process(source, config)

main()

```
