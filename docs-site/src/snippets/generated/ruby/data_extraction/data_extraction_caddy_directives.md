---
id: fixture_ruby_data_extraction_caddy_directives
language: ruby
target: ruby
level: typecheck
requires: []
side_effect: safe
---

```ruby title="Ruby"
require "tree_sitter_language_pack"
result = TreeSitterLanguagePack.process("localhost\nroot * /var/www\nfile_server\n", { 'data_extraction' => true, 'language' => 'caddy' })

```
