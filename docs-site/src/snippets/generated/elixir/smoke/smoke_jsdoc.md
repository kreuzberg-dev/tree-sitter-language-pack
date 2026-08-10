```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "jsdoc"}
result = TreeSitterLanguagePack.process("/** @param {string} name */", config_value)

```
