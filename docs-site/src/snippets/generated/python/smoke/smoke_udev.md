```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'ACTION=="add", KERNEL=="sd*"'
    config = {"language": "udev"}
    _ = process(source, config)

main()

```
