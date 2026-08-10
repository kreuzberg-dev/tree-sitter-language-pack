```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "*.foreground: #ffffff\n"
    config = {"language": "xresources"}
    _ = process(source, config)

main()

```
