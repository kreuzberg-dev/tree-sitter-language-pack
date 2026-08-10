```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'IO.puts("hello")'
    config = {"language": "elixir"}
    _ = process(source, config)

main()

```
