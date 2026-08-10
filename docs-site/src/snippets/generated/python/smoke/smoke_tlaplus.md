```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "---- MODULE Main ----\n===="
    config = {"language": "tlaplus"}
    _ = process(source, config)

main()

```
