langs = TreeSitterLanguagePack.available_languages()

if length(langs) == 0 do
  raise "no languages available"
end

IO.puts("Available languages: #{length(langs)}")

unless TreeSitterLanguagePack.has_language("elixir") do
  raise "elixir not found"
end

IO.puts("Elixir smoke test passed")
