```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "asciidoc"}
result = TreeSitterLanguagePack.process("= Title\n\nParagraph.", config_value)

```
