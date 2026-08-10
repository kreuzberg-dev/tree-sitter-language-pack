```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "fn main() {\n  1\n}\n"
    config = {"language": "aiken"}
    _ = process(source, config)

main()

```
