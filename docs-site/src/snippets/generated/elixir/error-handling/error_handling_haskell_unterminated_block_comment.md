```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "haskell"}
result = TreeSitterLanguagePack.process("{-aaaaaaaaaaaaaa aaaa}\n    {-aaa (aaaaaaaaaa [aaaaaaaaaaaaa aaa", config_value)

```
