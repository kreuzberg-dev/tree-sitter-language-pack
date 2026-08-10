```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'variable "name" { type = string }'
    config = {"language": "hcl"}
    _ = process(source, config)

main()

```
