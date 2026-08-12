---
id: fixture_elixir_error_handling_get_language_empty_string
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
try do
  language = TreeSitterLanguagePack.get_language("")
rescue
  error -> IO.puts(:stderr, "Call failed as expected: #{Exception.message(error)}")
else
  _ -> raise "expected call to fail"
end

```
