---
id: fixture_elixir_process_python_metrics_detail
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "python"}
result = TreeSitterLanguagePack.process("\# module docstring\nimport os\n\ndef hello():\n    \# greeting\n    print('hello')\n\ndef world():\n    print('world')\n", config_value)

```
