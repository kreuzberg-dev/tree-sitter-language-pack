```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'open("/x", O_RDONLY) = 3\n'
    config = {"language": "strace"}
    _ = process(source, config)

main()

```
