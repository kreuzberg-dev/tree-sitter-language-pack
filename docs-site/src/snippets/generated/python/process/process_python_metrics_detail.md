```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "# module docstring\nimport os\n\ndef hello():\n    # greeting\n    print('hello')\n\ndef world():\n    print('world')\n"
    config = {"language": "python"}
    _ = process(source, config)

main()

```
