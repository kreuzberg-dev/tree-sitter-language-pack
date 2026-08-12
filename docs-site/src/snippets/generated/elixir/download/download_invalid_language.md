---
id: fixture_elixir_download_invalid_language
language: elixir
target: elixir
level: typecheck
requires: []
side_effect: safe
---

```elixir title="Elixir"
try do
  result = TreeSitterLanguagePack.download(["zzz_definitely_not_a_real_language_xyz"])
rescue
  error -> IO.puts(:stderr, "Call failed as expected: #{Exception.message(error)}")
else
  _ -> raise "expected call to fail"
end

```
