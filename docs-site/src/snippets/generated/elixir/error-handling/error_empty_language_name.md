---
id: fixture_elixir_error_empty_language_name
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
try do
  config_value = %TreeSitterLanguagePack.ProcessConfig{language: ""}
  result = TreeSitterLanguagePack.process("hello", config_value)
rescue
  error -> IO.puts(:stderr, "Call failed as expected: #{Exception.message(error)}")
else
  _ -> raise "expected call to fail"
end

```
