---
id: fixture_elixir_download_configure_custom_dir
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.PackConfig{cache_dir: "/tmp/tslp_test_cache"}
TreeSitterLanguagePack.configure(config_value)

```
