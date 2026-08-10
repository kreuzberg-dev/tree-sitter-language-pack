```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "matlab"}
result = TreeSitterLanguagePack.process("function y = hello(x)\ny = x;\nend", config_value)

```
