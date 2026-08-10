```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "2024-01-01 open Assets:Bank USD"
    config = {"language": "beancount"}
    _ = process(source, config)

main()

```
