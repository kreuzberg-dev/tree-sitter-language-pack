```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "program Hello; begin end."
    config = {"language": "pascal"}
    _ = process(source, config)

main()

```
