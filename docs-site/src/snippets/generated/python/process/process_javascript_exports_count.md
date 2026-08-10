```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "export function greet() { return 'hi'; }\nexport const VERSION = '1.0';\nexport default class App {}\n"
    config = {"language": "javascript"}
    _ = process(source, config)

main()

```
