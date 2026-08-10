```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'object "C" {\n  code {\n  }\n}\n'
    config = {"language": "yul"}
    _ = process(source, config)

main()

```
