```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{chunk_max_size: 30, language: "rust"}
result = TreeSitterLanguagePack.process("fn alpha() {}\n\nfn beta() {}\n\nfn gamma() {}\n\nfn delta() {}\n", config_value)

```
