```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "_method object.hello\n_endmethod"
    config = {"language": "magik"}
    _ = process(source, config)

main()

```
