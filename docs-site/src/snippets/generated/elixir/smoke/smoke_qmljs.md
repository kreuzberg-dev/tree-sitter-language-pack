```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "qmljs"}
result = TreeSitterLanguagePack.process("import QtQuick 2.0\nItem {}", config_value)

```
