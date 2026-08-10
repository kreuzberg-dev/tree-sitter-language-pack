```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "nginx"}
result = TreeSitterLanguagePack.process("worker_processes 4;\nerror_log /var/log/nginx/error.log;\n", config_value)

```
