```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "class FOO\nend\n"
    config = {"language": "eiffel"}
    _ = process(source, config)

main()

```
