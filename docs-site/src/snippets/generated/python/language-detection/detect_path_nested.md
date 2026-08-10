```python title="Python"
from tree_sitter_language_pack import detect_language_from_path

def main() -> None:
    path = "src/main.py"
    _ = detect_language_from_path(path)

main()

```
