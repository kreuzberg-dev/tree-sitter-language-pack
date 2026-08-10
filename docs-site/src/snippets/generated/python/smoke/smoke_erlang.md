```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "main() -> ok."
    config = {"language": "erlang"}
    _ = process(source, config)

main()

```
