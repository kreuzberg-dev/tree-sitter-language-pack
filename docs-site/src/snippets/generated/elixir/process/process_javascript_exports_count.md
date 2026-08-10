```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "javascript"}
result = TreeSitterLanguagePack.process("export function greet() { return 'hi'; }\nexport const VERSION = '1.0';\nexport default class App {}\n", config_value)

```
