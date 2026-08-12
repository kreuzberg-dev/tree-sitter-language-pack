---
id: fixture_elixir_c_function_process
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "c"}
result = TreeSitterLanguagePack.process("\#include <stdio.h>\n\nint main() {\n    printf(\"hello\");\n    return 0;\n}\n", config_value)

```
