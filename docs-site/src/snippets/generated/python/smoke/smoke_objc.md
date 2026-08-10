```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "@interface Main @end"
    config = {"language": "objc"}
    _ = process(source, config)

main()

```
