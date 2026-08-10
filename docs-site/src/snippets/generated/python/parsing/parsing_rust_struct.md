```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "struct Point { x: f64, y: f64 }"
    config = {"language": "rust"}
    _ = process(source, config)

main()

```
