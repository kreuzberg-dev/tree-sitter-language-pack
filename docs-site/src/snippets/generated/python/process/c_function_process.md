```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = '#include <stdio.h>\n\nint main() {\n    printf("hello");\n    return 0;\n}\n'
    config = {"language": "c"}
    _ = process(source, config)

main()

```
