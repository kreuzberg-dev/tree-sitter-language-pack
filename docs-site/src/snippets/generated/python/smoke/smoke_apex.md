```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "public class Main {}"
    config = {"language": "apex"}
    _ = process(source, config)

main()

```
