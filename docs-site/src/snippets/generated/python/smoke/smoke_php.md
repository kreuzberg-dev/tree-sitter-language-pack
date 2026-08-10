```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "<?php echo 'hello'; ?>"
    config = {"language": "php"}
    _ = process(source, config)

main()

```
