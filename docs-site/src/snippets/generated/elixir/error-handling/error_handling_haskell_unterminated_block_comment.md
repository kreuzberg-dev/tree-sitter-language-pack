---
id: fixture_elixir_error_handling_haskell_unterminated_block_comment
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
config_value = %TreeSitterLanguagePack.ProcessConfig{language: "haskell"}
result = TreeSitterLanguagePack.process("{-aaaaaaaaaaaaaa aaaa}\n    {-aaa (aaaaaaaaaa [aaaaaaaaaaaaa aaa", config_value)

```
