```ruby title="Ruby"
require "tree_sitter_language_pack"

result = TreeSitterLanguagePack.process(
  "require 'json'\ndef parse(data)\n  JSON.parse(data)\nend",
  '{"language": "ruby", "structure": true, "imports": true}'
)

result["structure"].each do |item|
  puts "#{item['kind']}: #{item['name']}"
end

result["imports"].each do |imp|
  puts "import: #{imp}"
end
```
