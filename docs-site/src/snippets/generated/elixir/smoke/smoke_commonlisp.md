---
id: fixture_elixir_smoke_commonlisp
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "commonlisp"}
result = TreeSitterLanguagePack.process("(defun hello () (print \"hello\"))", config_value)

```
