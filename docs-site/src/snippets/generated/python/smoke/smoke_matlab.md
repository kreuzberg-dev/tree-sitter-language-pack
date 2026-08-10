```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "function y = hello(x)\ny = x;\nend"
    config = {"language": "matlab"}
    _ = process(source, config)

main()

```
