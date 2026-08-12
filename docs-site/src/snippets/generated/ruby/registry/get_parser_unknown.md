---
id: fixture_ruby_get_parser_unknown
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
begin
  parser = TreeSitterLanguagePack.get_parser('nonexistent_xyz')
rescue StandardError => error
  warn "Call failed as expected: #{error.message}"
else
  raise "expected call to fail"
end

```
