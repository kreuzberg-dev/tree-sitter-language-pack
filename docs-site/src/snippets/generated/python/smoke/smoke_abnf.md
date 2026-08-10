```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'a = "b"\r\n'
    config = {"language": "abnf"}
    _ = process(source, config)

main()

```
