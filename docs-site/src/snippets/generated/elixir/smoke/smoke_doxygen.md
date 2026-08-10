```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "doxygen"}
result = TreeSitterLanguagePack.process("/** @brief A function */", config_value)

```
