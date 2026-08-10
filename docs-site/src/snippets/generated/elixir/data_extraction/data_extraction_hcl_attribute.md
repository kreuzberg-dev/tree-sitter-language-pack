```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "hcl"}
result = TreeSitterLanguagePack.process("region = \"us-east-1\"\ncount  = 3\n", config_value)

```
