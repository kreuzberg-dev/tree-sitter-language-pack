```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = '(defwidget main [] (label :text "hi"))'
    config = {"language": "yuck"}
    _ = process(source, config)

main()

```
