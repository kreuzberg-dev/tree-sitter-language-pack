```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "SELECT ?s WHERE { ?s ?p ?o }"
    config = {"language": "sparql"}
    _ = process(source, config)

main()

```
