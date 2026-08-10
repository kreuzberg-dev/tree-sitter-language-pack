```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "protocol P {\n}\n"
    config = {"language": "avro"}
    _ = process(source, config)

main()

```
