```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "cobol"}
result = TreeSitterLanguagePack.process("       IDENTIFICATION DIVISION.\n       PROGRAM-ID. HELLO.", config_value)

```
