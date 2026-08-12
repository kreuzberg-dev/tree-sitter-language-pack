---
id: fixture_ruby_download_invalid_language
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
begin
  result = TreeSitterLanguagePack.download(['zzz_definitely_not_a_real_language_xyz'])
rescue StandardError => error
  warn "Call failed as expected: #{error.message}"
else
  raise "expected call to fail"
end

```
