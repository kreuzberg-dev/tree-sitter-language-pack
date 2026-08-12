---
id: fixture_elixir_get_parser_unknown
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
try do
  parser = TreeSitterLanguagePack.get_parser("nonexistent_xyz")
rescue
  error -> IO.puts(:stderr, "Call failed as expected: #{Exception.message(error)}")
else
  _ -> raise "expected call to fail"
end

```
