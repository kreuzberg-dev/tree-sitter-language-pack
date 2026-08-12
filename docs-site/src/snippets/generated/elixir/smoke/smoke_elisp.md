---
id: fixture_elixir_smoke_elisp
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "elisp"}
result = TreeSitterLanguagePack.process("(defun hello () (message \"hello\"))", config_value)

```
