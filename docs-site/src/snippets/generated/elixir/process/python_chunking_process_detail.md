---
id: fixture_elixir_python_chunking_process_detail
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{chunk_max_size: 30, language: "python"}
result = TreeSitterLanguagePack.process("def alpha():\n    pass\n\ndef beta():\n    pass\n\ndef gamma():\n    pass\n\ndef delta():\n    pass\n", config_value)

```
