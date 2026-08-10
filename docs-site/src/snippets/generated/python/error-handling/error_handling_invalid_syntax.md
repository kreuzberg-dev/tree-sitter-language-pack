```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "function function function @@@ %%%"
    config = {"language": "javascript"}
    _ = process(source, config)

main()

```
