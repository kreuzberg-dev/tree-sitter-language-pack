---
id: fixture_ruby_error_handling_get_language_empty_string
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
begin
  language = TreeSitterLanguagePack.get_language('')
rescue StandardError => error
  warn "Call failed as expected: #{error.message}"
else
  raise "expected call to fail"
end

```
