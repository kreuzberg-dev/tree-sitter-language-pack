```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'def greet(name):\n    """Say hello to someone."""\n    return f"Hello {name}"\n'
    config = {"docstrings": True, "language": "python"}
    _ = process(source, config)

main()

```
