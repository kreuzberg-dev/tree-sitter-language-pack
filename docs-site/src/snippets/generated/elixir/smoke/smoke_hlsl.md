```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "hlsl"}
result = TreeSitterLanguagePack.process("float4 main() : SV_Target { return 0; }", config_value)

```
