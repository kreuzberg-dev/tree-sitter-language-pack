```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "kotlin"}
result = TreeSitterLanguagePack.process("package foo.bar\n\nclass Widget {\n    fun greet(): String = \"hi\"\n}\n", config_value)

```
