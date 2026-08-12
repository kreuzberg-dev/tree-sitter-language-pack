---
id: fixture_ruby_detect_content_no_shebang
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.detect_language_from_content('no shebang here')

```
