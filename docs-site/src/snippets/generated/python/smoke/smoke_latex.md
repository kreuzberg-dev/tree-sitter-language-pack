---
id: fixture_python_smoke_latex
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}"
    config = {"language": "latex"}
    _ = process(source, config)

main()

```
