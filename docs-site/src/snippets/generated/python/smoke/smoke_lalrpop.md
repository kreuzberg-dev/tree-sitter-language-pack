```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "grammar;\n"
    config = {"language": "lalrpop"}
    _ = process(source, config)

main()

```
