---
id: fixture_python_data_extraction_dtd_element_decl
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "<!ELEMENT server (host, port)>\n<!ELEMENT host (#PCDATA)>\n"
    config = {"data_extraction": True, "language": "dtd"}
    _ = process(source, config)

main()

```
