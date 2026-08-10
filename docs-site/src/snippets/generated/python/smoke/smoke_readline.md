```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "set editing-mode vi"
    config = {"language": "readline"}
    _ = process(source, config)

main()

```
