---
id: fixture_elixir_detect_content_bash_shebang
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
result = TreeSitterLanguagePack.detect_language_from_content("\#!/bin/bash\necho hi")

```
