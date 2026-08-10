```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "objc"}
result = TreeSitterLanguagePack.process("@interface Main @end", config_value)

```
