```python title="Python"
from tree_sitter_language_pack import get_parser

def main() -> None:
    name = "nonexistent_xyz"
    _ = get_parser(name)

main()

```
