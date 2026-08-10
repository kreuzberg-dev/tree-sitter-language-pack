```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "dot"}
result = TreeSitterLanguagePack.process("digraph G { A -> B; }", config_value)

```
