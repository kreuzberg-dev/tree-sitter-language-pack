```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "pub fn main() void {}"
    config = {"language": "zig"}
    _ = process(source, config)

main()

```
