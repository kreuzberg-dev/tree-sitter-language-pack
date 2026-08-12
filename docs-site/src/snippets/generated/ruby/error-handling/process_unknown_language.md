---
id: fixture_ruby_process_unknown_language
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
begin
  result = TreeSitterLanguagePack.process('x = 1', { 'language' => 'nonexistent_xyz' })
rescue StandardError => error
  warn "Call failed as expected: #{error.message}"
else
  raise "expected call to fail"
end

```
