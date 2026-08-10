```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "def first():\n    x = 1\n    return x\n\ndef second():\n    y = 2\n    return y\n\ndef third():\n    z = 3\n    return z\n"
    config = {"chunk_max_size": 50, "language": "python"}
    _ = process(source, config)

main()

```
