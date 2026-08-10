```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "vue"}
result = TreeSitterLanguagePack.process("<template><div>hello</div></template>", config_value)

```
