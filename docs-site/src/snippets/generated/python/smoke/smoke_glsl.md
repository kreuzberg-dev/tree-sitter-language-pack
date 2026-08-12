---
id: fixture_python_smoke_glsl
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "void main() { gl_Position = vec4(0.0); }"
    config = {"language": "glsl"}
    _ = process(source, config)

main()

```
