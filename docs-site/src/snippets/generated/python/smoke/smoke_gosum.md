```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "example.com/pkg v1.0.0 h1:abc="
    config = {"language": "gosum"}
    _ = process(source, config)

main()

```
