```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "%token EOF\n%%\n"
    config = {"language": "menhir"}
    _ = process(source, config)

main()

```
