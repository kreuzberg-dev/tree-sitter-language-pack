---
id: fixture_elixir_smoke_qmljs
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "qmljs"}
result = TreeSitterLanguagePack.process("import QtQuick 2.0\nItem {}", config_value)

```
