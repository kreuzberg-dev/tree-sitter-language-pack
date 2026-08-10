```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "model User { id Int @id }"
    config = {"language": "prisma"}
    _ = process(source, config)

main()

```
