---
id: fixture_python_data_extraction_caddy_directives
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "localhost\nroot * /var/www\nfile_server\n"
    config = {"data_extraction": True, "language": "caddy"}
    _ = process(source, config)

main()

```
