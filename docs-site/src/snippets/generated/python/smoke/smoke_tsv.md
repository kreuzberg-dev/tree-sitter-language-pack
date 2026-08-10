```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "a\tb\tc\n1\t2\t3"
    config = {"language": "tsv"}
    _ = process(source, config)

main()

```
