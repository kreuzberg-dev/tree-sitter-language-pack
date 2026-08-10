```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "fortran"}
result = TreeSitterLanguagePack.process("program main\nend program main", config_value)

```
