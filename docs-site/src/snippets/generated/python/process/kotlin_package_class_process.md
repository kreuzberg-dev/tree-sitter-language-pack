```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'package foo.bar\n\nclass Widget {\n    fun greet(): String = "hi"\n}\n'
    config = {"language": "kotlin"}
    _ = process(source, config)

main()

```
