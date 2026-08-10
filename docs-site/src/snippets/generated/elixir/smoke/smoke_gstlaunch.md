```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "gstlaunch"}
result = TreeSitterLanguagePack.process("fakesrc ! fakesink", config_value)

```
