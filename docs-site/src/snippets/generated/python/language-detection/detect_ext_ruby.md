```python title="Python"
from tree_sitter_language_pack import detect_language_from_extension

def main() -> None:
    ext = "rb"
    _ = detect_language_from_extension(ext)

main()

```
