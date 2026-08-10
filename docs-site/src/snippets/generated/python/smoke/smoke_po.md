```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'msgid "hello"\nmsgstr "world"'
    config = {"language": "po"}
    _ = process(source, config)

main()

```
