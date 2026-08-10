```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "notify { 'hello': }"
    config = {"language": "puppet"}
    _ = process(source, config)

main()

```
