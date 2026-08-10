```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "/** @param {string} name */"
    config = {"language": "jsdoc"}
    _ = process(source, config)

main()

```
