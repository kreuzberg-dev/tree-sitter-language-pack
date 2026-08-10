```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "-----BEGIN CERTIFICATE-----\ndata\n-----END CERTIFICATE-----"
    config = {"language": "pem"}
    _ = process(source, config)

main()

```
