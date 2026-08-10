```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "Host example\n  HostName example.com"
    config = {"language": "ssh_config"}
    _ = process(source, config)

main()

```
