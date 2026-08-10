```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "BEGIN { }\n"
    config = {"language": "bpftrace"}
    _ = process(source, config)

main()

```
