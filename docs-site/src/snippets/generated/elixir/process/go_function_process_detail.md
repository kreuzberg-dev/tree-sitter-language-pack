```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "go"}
result = TreeSitterLanguagePack.process("package main\n\nimport \"fmt\"\n\nfunc main() {\n\tfmt.Println(\"hello\")\n}\n", config_value)

```
