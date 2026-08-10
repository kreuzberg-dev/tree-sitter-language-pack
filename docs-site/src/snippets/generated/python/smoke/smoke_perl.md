```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "print 'hello';"
    config = {"language": "perl"}
    _ = process(source, config)

main()

```
