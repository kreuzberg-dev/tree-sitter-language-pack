```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "export fn main() void = void;"
    config = {"language": "hare"}
    _ = process(source, config)

main()

```
