```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = 'resource "null_resource" "main" {}'
    config = {"language": "terraform"}
    _ = process(source, config)

main()

```
