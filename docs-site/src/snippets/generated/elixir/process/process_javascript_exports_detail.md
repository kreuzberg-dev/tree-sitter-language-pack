```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "javascript"}
result = TreeSitterLanguagePack.process("export function greet(name) {\n  return `Hello ${name}`;\n}\n\nexport const VERSION = '1.0';\n", config_value)

```
