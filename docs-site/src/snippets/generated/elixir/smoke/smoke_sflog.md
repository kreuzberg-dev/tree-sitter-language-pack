```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "sflog"}
result = TreeSitterLanguagePack.process("37.0 APEX_CODE,DEBUG\n16:06:58.18 (1)|EXECUTION_STARTED\n", config_value)

```
