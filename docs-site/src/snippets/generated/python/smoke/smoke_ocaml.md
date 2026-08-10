```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'let () = print_endline "hello"'
    config = {"language": "ocaml"}
    _ = process(source, config)

main()

```
