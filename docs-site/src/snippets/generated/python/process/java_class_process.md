---
id: fixture_python_java_class_process
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'import java.util.List;\n\npublic class Greeter {\n    public String greet(String name) {\n        return "Hello " + name;\n    }\n}\n'
    config = {"language": "java"}
    _ = process(source, config)

main()

```
