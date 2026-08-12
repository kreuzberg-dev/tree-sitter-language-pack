---
id: fixture_elixir_process_python_imports_detail
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "python"}
result = TreeSitterLanguagePack.process("import os\nimport sys\nfrom pathlib import Path\n\ndef main():\n    pass\n", config_value)

```
