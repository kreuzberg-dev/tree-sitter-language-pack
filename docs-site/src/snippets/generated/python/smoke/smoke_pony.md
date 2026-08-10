```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "actor Main\n  new create(env: Env) => None"
    config = {"language": "pony"}
    _ = process(source, config)

main()

```
