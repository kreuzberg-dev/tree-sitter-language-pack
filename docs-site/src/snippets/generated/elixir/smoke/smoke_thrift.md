```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "thrift"}
result = TreeSitterLanguagePack.process("service HelloService {}", config_value)

```
