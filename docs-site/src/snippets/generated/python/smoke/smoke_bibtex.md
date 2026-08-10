```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "@article{key, title={A}}"
    config = {"language": "bibtex"}
    _ = process(source, config)

main()

```
