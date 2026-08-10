```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "class Main { static function main() {} }"
    config = {"language": "haxe"}
    _ = process(source, config)

main()

```
