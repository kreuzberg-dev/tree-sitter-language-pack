```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "init {\n}\n"
    config = {"language": "promela"}
    _ = process(source, config)

main()

```
