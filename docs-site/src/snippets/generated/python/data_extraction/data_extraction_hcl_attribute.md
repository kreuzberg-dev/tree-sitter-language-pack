```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'region = "us-east-1"\ncount  = 3\n'
    config = {"data_extraction": True, "language": "hcl"}
    _ = process(source, config)

main()

```
