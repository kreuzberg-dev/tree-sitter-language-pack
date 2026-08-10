```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "\\documentclass{article}\n\\begin{document}\nHello\n\\end{document}"
    config = {"language": "latex"}
    _ = process(source, config)

main()

```
