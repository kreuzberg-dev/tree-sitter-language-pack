```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = ':8080 {\n\trespond "Hello"\n}'
    config = {"language": "caddy"}
    _ = process(source, config)

main()

```
