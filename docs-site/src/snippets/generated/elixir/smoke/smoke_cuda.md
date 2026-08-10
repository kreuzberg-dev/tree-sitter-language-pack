```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "cuda"}
result = TreeSitterLanguagePack.process("__global__ void kernel() {}", config_value)

```
