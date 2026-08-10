```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "T | count\n"
    config = {"language": "kusto"}
    _ = process(source, config)

main()

```
