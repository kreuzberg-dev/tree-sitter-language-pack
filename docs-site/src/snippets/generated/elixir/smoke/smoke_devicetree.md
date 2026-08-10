```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "devicetree"}
result = TreeSitterLanguagePack.process("/dts-v1/;\n/ { };", config_value)

```
