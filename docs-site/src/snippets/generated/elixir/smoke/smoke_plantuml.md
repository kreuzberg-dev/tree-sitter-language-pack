```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "plantuml"}
result = TreeSitterLanguagePack.process("@startuml\n@enduml\n", config_value)

```
