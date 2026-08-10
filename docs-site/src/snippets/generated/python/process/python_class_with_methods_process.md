```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "class Calculator:\n    def add(self, a, b):\n        return a + b\n\n    def subtract(self, a, b):\n        return a - b\n"
    config = {"language": "python"}
    _ = process(source, config)

main()

```
