---
id: fixture_elixir_smoke_php
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "php"}
result = TreeSitterLanguagePack.process("<?php echo 'hello'; ?>", config_value)

```
