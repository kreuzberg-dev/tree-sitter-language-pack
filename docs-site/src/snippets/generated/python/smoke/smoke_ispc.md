```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "export void main() {}"
    config = {"language": "ispc"}
    _ = process(source, config)

main()

```
