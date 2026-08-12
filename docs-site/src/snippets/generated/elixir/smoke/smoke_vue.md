---
id: fixture_elixir_smoke_vue
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "vue"}
result = TreeSitterLanguagePack.process("<template><div>hello</div></template>", config_value)

```
