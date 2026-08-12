---
id: fixture_python_data_extraction_nginx_directives
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "worker_processes 4;\nerror_log /var/log/nginx/error.log;\n"
    config = {"data_extraction": True, "language": "nginx"}
    _ = process(source, config)

main()

```
