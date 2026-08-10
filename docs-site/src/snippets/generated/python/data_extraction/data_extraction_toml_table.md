```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = '[server]\nhost = "localhost"\nport = 8080\n'
    config = {"data_extraction": True, "language": "toml"}
    _ = process(source, config)

main()

```
