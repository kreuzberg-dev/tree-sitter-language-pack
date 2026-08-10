```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "/*!re2c\n  [a-z]+ { return; }\n*/"
    config = {"language": "re2c"}
    _ = process(source, config)

main()

```
