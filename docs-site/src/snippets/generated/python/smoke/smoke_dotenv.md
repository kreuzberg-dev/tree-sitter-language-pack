```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "KEY=value\n"
    config = {"language": "dotenv"}
    _ = process(source, config)

main()

```
