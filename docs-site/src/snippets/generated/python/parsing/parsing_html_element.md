```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "<div>hello</div>"
    config = {"language": "html"}
    _ = process(source, config)

main()

```
