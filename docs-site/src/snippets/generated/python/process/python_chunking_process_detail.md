```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "def alpha():\n    pass\n\ndef beta():\n    pass\n\ndef gamma():\n    pass\n\ndef delta():\n    pass\n"
    config = {"chunk_max_size": 30, "language": "python"}
    _ = process(source, config)

main()

```
