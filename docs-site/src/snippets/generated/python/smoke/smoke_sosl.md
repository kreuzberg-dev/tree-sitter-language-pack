```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "FIND {test}\n"
    config = {"language": "sosl"}
    _ = process(source, config)

main()

```
