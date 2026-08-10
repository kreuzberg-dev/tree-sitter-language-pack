```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "data _null_;\nrun;\n"
    config = {"language": "sas"}
    _ = process(source, config)

main()

```
