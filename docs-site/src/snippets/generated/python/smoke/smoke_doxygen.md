```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "/** @brief A function */"
    config = {"language": "doxygen"}
    _ = process(source, config)

main()

```
