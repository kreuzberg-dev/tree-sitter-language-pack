```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "ports:\n  - 8080\n  - 8081\n"
    config = {"data_extraction": True, "language": "yaml"}
    _ = process(source, config)

main()

```
