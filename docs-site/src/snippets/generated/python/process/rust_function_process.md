```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "fn add(a: i32, b: i32) -> i32 {\n    a + b\n}\n"
    config = {"language": "rust"}
    _ = process(source, config)

main()

```
