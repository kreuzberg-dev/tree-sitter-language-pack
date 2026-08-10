```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "program test.aleo {\n}\n"
    config = {"language": "leo"}
    _ = process(source, config)

main()

```
