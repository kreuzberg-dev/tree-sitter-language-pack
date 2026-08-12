---
id: fixture_python_ruby_class_process
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "require 'json'\n\nclass Greeter\n  def greet(name)\n    \"Hello #{name}\"\n  end\nend\n"
    config = {"language": "ruby"}
    _ = process(source, config)

main()

```
