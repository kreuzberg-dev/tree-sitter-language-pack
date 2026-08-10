```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'package com.example.widget;\n\npublic class Widget {\n    public String name() { return "w"; }\n}\n'
    config = {"language": "java"}
    _ = process(source, config)

main()

```
