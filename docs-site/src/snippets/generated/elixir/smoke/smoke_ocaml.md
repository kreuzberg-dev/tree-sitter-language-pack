```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ocaml"}
result = TreeSitterLanguagePack.process("let () = print_endline \"hello\"", config_value)

```
