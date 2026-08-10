```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "linkerscript"}
result = TreeSitterLanguagePack.process("SECTIONS { .text : { *(.text) } }", config_value)

```
