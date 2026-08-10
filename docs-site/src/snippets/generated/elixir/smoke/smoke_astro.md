```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "astro"}
result = TreeSitterLanguagePack.process("---\n---\n<p>hello</p>", config_value)

```
