```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "fn main() {}"
    config = {"language": "rust"}
    _ = process(source, config)

main()

```
