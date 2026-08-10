```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "dtd"}
result = TreeSitterLanguagePack.process("<!ELEMENT note (body)>", config_value)

```
