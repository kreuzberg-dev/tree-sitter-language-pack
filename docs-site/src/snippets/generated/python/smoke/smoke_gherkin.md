---
id: fixture_python_smoke_gherkin
language: python
target: python
level: typecheck
requires: []
side_effect: safe
---

```python title="Python"
from tree_sitter_language_pack import process

def main() -> None:
    source = "Feature: Calculator\n  Scenario: Add numbers\n    Given I have entered 1\n    When I add 2\n    Then the result should be 3\n"
    config = {"language": "gherkin"}
    _ = process(source, config)

main()

```
