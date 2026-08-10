```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "var x:int = 1;"
    config = {"language": "actionscript"}
    _ = process(source, config)

main()

```
