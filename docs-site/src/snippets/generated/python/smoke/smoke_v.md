```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "fn main() {}"
    config = {"language": "v"}
    _ = process(source, config)

main()

```
