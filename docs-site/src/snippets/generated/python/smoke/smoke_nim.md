```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'echo "hello"'
    config = {"language": "nim"}
    _ = process(source, config)

main()

```
