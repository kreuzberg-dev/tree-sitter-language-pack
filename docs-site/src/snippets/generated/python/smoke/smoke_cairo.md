```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "fn main() {}"
    config = {"language": "cairo"}
    _ = process(source, config)

main()

```
