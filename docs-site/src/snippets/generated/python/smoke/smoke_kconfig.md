```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'config FOO\n\tbool "Enable foo"'
    config = {"language": "kconfig"}
    _ = process(source, config)

main()

```
