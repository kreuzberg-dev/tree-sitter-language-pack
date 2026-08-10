```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "python"}
result = TreeSitterLanguagePack.process("class Calculator:\n    def add(self, a, b):\n        return a + b\n\n    def subtract(self, a, b):\n        return a - b\n", config_value)

```
