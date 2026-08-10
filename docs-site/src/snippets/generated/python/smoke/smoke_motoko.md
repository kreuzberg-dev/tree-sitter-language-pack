```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "actor {\n}\n"
    config = {"language": "motoko"}
    _ = process(source, config)

main()

```
