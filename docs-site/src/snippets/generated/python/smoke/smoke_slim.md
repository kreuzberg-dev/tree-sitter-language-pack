```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "p hello\n"
    config = {"language": "slim"}
    _ = process(source, config)

main()

```
