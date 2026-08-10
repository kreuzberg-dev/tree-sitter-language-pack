```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "package P {}\n"
    config = {"language": "sysml"}
    _ = process(source, config)

main()

```
