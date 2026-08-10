```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "solidity"}
result = TreeSitterLanguagePack.process("pragma solidity ^0.8.0;\ncontract Main {}", config_value)

```
