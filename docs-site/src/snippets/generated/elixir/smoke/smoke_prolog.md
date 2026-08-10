```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "prolog"}
result = TreeSitterLanguagePack.process("hello :- write('hello'), nl.", config_value)

```
