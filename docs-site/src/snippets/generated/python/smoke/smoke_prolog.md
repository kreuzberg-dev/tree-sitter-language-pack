```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "hello :- write('hello'), nl."
    config = {"language": "prolog"}
    _ = process(source, config)

main()

```
