```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "%[greeting]\n    hello"
    config = {"language": "chatito"}
    _ = process(source, config)

main()

```
