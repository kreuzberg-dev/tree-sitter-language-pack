---
id: fixture_elixir_data_extraction_caddy_directives
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{data_extraction: true, language: "caddy"}
result = TreeSitterLanguagePack.process("localhost\nroot * /var/www\nfile_server\n", config_value)

```
