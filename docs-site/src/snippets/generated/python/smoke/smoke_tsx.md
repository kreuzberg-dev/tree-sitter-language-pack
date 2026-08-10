```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "const App = () => <div />;"
    config = {"language": "tsx"}
    _ = process(source, config)

main()

```
