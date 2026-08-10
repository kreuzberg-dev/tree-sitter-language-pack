```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "PROGRAM_NAME='hello'"
    config = {"language": "netlinx"}
    _ = process(source, config)

main()

```
