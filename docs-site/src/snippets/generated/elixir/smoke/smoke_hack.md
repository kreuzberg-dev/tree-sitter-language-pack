```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "hack"}
result = TreeSitterLanguagePack.process("<?hh\nfunction main(): void {}", config_value)

```
