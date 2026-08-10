```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "class Foo { bar() {} }"
    config = {"language": "javascript"}
    _ = process(source, config)

main()

```
