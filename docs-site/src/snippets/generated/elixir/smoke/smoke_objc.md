---
id: fixture_elixir_smoke_objc
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "objc"}
result = TreeSitterLanguagePack.process("@interface Main @end", config_value)

```
