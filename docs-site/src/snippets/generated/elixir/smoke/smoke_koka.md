```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "koka"}
result = TreeSitterLanguagePack.process("fun main()\n  1\n", config_value)

```
