```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "php"}
result = TreeSitterLanguagePack.process("<?php echo 'hello'; ?>", config_value)

```
