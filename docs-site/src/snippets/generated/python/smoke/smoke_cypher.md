```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "MATCH (n) RETURN n\n"
    config = {"language": "cypher"}
    _ = process(source, config)

main()

```
