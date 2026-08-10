```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "cypher"}
result = TreeSitterLanguagePack.process("MATCH (n) RETURN n\n", config_value)

```
