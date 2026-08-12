---
id: fixture_python_parsing_html_element
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "<div>hello</div>"
    config = {"language": "html"}
    _ = process(source, config)

main()

```
