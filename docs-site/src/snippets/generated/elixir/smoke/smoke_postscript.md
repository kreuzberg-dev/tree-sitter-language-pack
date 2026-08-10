```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "postscript"}
result = TreeSitterLanguagePack.process("/hello { (Hello) show } def", config_value)

```
