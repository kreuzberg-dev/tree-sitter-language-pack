```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "SELECT 1;\n"
    config = {"language": "postgres"}
    _ = process(source, config)

main()

```
