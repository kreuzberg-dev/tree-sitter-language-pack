```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "{-aaaaaaaaaaaaaa aaaa}\n    {-aaa (aaaaaaaaaa [aaaaaaaaaaaaa aaa"
    config = {"language": "haskell"}
    _ = process(source, config)

main()

```
