```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = '(defun hello () (message "hello"))'
    config = {"language": "elisp"}
    _ = process(source, config)

main()

```
