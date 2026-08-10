```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "cmake"}
result = TreeSitterLanguagePack.process("cmake_minimum_required(VERSION 3.0)", config_value)

```
