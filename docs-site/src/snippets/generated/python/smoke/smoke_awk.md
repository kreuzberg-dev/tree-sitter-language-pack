```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'BEGIN { print "hello" }'
    config = {"language": "awk"}
    _ = process(source, config)

main()

```
