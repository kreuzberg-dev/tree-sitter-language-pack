```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "---@param name string"
    config = {"language": "luadoc"}
    _ = process(source, config)

main()

```
