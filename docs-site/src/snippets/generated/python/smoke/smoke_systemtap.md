```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "probe begin {}\n"
    config = {"language": "systemtap"}
    _ = process(source, config)

main()

```
