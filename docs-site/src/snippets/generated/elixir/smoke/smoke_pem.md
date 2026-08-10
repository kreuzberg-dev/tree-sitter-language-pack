```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "pem"}
result = TreeSitterLanguagePack.process("-----BEGIN CERTIFICATE-----\ndata\n-----END CERTIFICATE-----", config_value)

```
