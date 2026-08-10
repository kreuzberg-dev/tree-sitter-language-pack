```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "class Main {}"
    config = {"language": "csharp"}
    _ = process(source, config)

main()

```
