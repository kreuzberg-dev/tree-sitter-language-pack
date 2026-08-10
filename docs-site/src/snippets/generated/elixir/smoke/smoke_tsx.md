```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "tsx"}
result = TreeSitterLanguagePack.process("const App = () => <div />;", config_value)

```
