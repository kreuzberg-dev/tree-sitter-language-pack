```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "caddy"}
result = TreeSitterLanguagePack.process("localhost\nroot * /var/www\nfile_server\n", config_value)

```
