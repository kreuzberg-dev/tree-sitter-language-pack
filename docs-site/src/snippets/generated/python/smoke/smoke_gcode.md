```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "G0 X0\n"
    config = {"language": "gcode"}
    _ = process(source, config)

main()

```
