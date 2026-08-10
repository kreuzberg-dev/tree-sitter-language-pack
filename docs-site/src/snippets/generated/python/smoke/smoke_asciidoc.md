```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "= Title\n\nParagraph."
    config = {"language": "asciidoc"}
    _ = process(source, config)

main()

```
