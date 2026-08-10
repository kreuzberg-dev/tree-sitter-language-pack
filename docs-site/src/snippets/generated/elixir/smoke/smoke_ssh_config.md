```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "ssh_config"}
result = TreeSitterLanguagePack.process("Host example\n  HostName example.com", config_value)

```
