```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "function greet(name: string): string { return `hi ${name}`; }"
    config = {"language": "typescript"}
    _ = process(source, config)

main()

```
