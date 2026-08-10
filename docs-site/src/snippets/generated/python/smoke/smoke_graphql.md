```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "type Query { hello: String }"
    config = {"language": "graphql"}
    _ = process(source, config)

main()

```
