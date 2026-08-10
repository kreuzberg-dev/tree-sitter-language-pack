```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "@vertex fn main() -> @builtin(position) vec4f { return vec4f(); }"
    config = {"language": "wgsl"}
    _ = process(source, config)

main()

```
