```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "bibtex"}
result = TreeSitterLanguagePack.process("@article{key, title={A}}", config_value)

```
