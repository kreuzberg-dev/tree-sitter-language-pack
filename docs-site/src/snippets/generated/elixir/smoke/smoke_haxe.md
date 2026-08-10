```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "haxe"}
result = TreeSitterLanguagePack.process("class Main { static function main() {} }", config_value)

```
