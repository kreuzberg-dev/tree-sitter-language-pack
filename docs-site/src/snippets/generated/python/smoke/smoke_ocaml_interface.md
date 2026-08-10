```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "val x : int"
    config = {"language": "ocaml_interface"}
    _ = process(source, config)

main()

```
