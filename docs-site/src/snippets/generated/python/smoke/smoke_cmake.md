```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "cmake_minimum_required(VERSION 3.0)"
    config = {"language": "cmake"}
    _ = process(source, config)

main()

```
