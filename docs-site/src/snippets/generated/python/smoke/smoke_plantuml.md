```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "@startuml\n@enduml\n"
    config = {"language": "plantuml"}
    _ = process(source, config)

main()

```
