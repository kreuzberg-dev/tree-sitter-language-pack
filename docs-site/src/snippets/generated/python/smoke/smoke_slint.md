```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "export component Foo {}\n"
    config = {"language": "slint"}
    _ = process(source, config)

main()

```
