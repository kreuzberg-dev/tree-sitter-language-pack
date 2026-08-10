```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "{{ variable }}"
    config = {"language": "twig"}
    _ = process(source, config)

main()

```
