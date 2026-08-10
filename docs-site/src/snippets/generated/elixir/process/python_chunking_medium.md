```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{chunk_max_size: 50, language: "python"}
result = TreeSitterLanguagePack.process("def first():\n    x = 1\n    return x\n\ndef second():\n    y = 2\n    return y\n\ndef third():\n    z = 3\n    return z\n", config_value)

```
